use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex, RwLock,
        atomic::{AtomicUsize, Ordering},
        mpsc::{
            self, Receiver, RecvError, RecvTimeoutError, Sender, SyncSender, TryRecvError,
            TrySendError,
        },
    },
    thread::JoinHandle,
    time::Duration,
};

use crate::core::{file::Lang, frozen_graph::FrozenSyntaxGraph, prng::PRNG};

pub const ERR_SHUT_DOWN: &str = "resrap: instance has been shut down";

/// The result of one generation job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeGenRes {
    pub code: String,
    pub id: String,
}

/// A cloneable, blocking result channel. Multiple consumers may call `recv`;
/// disconnected means `shut_down_resrap` has finished draining every result.
#[derive(Clone)]
pub struct CodeChannel {
    rx: Arc<Mutex<Receiver<CodeGenRes>>>,
}

impl CodeChannel {
    pub fn recv(&self) -> Result<CodeGenRes, RecvError> {
        self.rx.lock().unwrap().recv()
    }

    pub fn try_recv(&self) -> Result<CodeGenRes, TryRecvError> {
        self.rx.lock().unwrap().try_recv()
    }

    pub fn recv_timeout(&self, timeout: Duration) -> Result<CodeGenRes, RecvTimeoutError> {
        self.rx.lock().unwrap().recv_timeout(timeout)
    }
}

struct CodeGenReq {
    /// Grammar snapshot resolved at submission time. This keeps the hot worker
    /// path free of grammar-map locks and name lookups.
    graph: Option<Arc<FrozenSyntaxGraph>>,
    start_id: Option<u32>,
    seed: u64,
    tokens: usize,
    complete: bool,
    id: String,
}

/// The multithreaded version of [`crate::Resrap`].
pub struct ResrapMT {
    language_graph: Arc<RwLock<HashMap<String, Lang>>>,
    pool_size: usize,
    wait_queue_size: usize,
    /// One bounded queue per worker. This avoids sharing a single
    /// `Receiver` through a mutex, which would serialise every job handoff.
    pending_tx: Vec<SyncSender<CodeGenReq>>,
    pending_rx: Option<Vec<Receiver<CodeGenReq>>>,
    next_worker: AtomicUsize,
    result_tx: Option<Sender<CodeGenRes>>,
    code_channel: CodeChannel,
    workers: Vec<JoinHandle<()>>,
    started: bool,
    closed: bool,
}

impl ResrapMT {
    /// Creates a new multithreaded instance. `pool_size` and `wait_queue_size`
    /// are clamped to at least one, matching the reference implementation. The
    /// wait queue is split evenly across the worker queues.
    pub fn new(pool_size: usize, wait_queue_size: usize) -> Self {
        let pool_size = pool_size.max(1);
        let wait_queue_size = wait_queue_size.max(1);
        let per_worker = wait_queue_size.div_ceil(pool_size).max(1);

        let mut pending_tx = Vec::with_capacity(pool_size);
        let mut pending_rx = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            let (tx, rx) = mpsc::sync_channel(per_worker);
            pending_tx.push(tx);
            pending_rx.push(rx);
        }

        let (result_tx, code_rx) = mpsc::channel::<CodeGenRes>();

        ResrapMT {
            language_graph: Arc::new(RwLock::new(HashMap::new())),
            pool_size,
            wait_queue_size,
            pending_tx,
            pending_rx: Some(pending_rx),
            next_worker: AtomicUsize::new(0),
            result_tx: Some(result_tx),
            code_channel: CodeChannel {
                rx: Arc::new(Mutex::new(code_rx)),
            },
            workers: Vec::new(),
            started: false,
            closed: false,
        }
    }

    pub fn pool_size(&self) -> usize {
        self.pool_size
    }

    pub fn wait_queue_size(&self) -> usize {
        self.wait_queue_size
    }

    /// The main endpoint for processed results.
    pub fn get_code_channel(&self) -> CodeChannel {
        self.code_channel.clone()
    }

    pub fn parse_grammar(&mut self, name: &str, grammar: &str) -> Result<(), String> {
        self.parse_grammar_impl(name, grammar)
    }

    fn parse_grammar_impl(&mut self, name: &str, grammar: &str) -> Result<(), String> {
        let mut lang = Lang::new();
        lang.parse_string(grammar)?;
        self.language_graph
            .write()
            .unwrap()
            .insert(name.to_string(), lang);
        Ok(())
    }

    pub fn parse_grammar_file(&mut self, name: &str, location: &str) -> Result<(), String> {
        let mut lang = Lang::new();
        lang.parse_file(location)?;
        self.language_graph
            .write()
            .unwrap()
            .insert(name.to_string(), lang);
        Ok(())
    }

    /// Attaches a list of words to a rule in a parsed grammar.
    pub fn add_words(&self, name: &str, rule: &str, words: &[String]) -> Result<(), String> {
        let graph = self.language_graph.read().unwrap();
        let lang = graph
            .get(name)
            .ok_or_else(|| format!("resrap: grammar '{}' is not loaded", name))?;
        let graph = lang
            .get_graph()
            .ok_or_else(|| format!("resrap: grammar '{}' is not loaded", name))?;
        graph.set_words(rule, words)
    }

    pub fn add_keywords(&self, name: &str, rule: &str, words: &[String]) -> Result<(), String> {
        self.add_words(name, rule, words)
    }

    fn submit(&self, req: CodeGenReq) -> Result<(), String> {
        if self.closed || self.pending_tx.is_empty() {
            return Err(ERR_SHUT_DOWN.to_string());
        }

        let len = self.pending_tx.len();
        let start = self.next_worker.fetch_add(1, Ordering::Relaxed) % len;
        let mut req = req;

        // Prefer an idle worker, but fall back to the round-robin target so a
        // slow or uneven workload still receives backpressure instead of
        // failing or growing an unbounded queue.
        for offset in 0..len {
            let index = (start + offset) % len;
            match self.pending_tx[index].try_send(req) {
                Ok(()) => return Ok(()),
                Err(TrySendError::Full(returned)) => req = returned,
                Err(TrySendError::Disconnected(_)) => {
                    return Err(ERR_SHUT_DOWN.to_string());
                }
            }
        }
        self.pending_tx[start]
            .send(req)
            .map_err(|_| ERR_SHUT_DOWN.to_string())
    }

    fn submit_job(
        &self,
        id: &str,
        name: &str,
        starting_node: &str,
        seed: u64,
        tokens: usize,
        complete: bool,
    ) -> Result<(), String> {
        let (graph, start_id) = {
            let languages = self.language_graph.read().unwrap();
            match languages.get(name).and_then(|lang| lang.get_graph_arc()) {
                Some(graph) => {
                    let start_id = graph.name_map.get(starting_node).copied();
                    (Some(graph), start_id)
                }
                None => (None, None),
            }
        };

        self.submit(CodeGenReq {
            graph,
            start_id,
            seed,
            tokens,
            complete,
            id: id.to_string(),
        })
    }

    pub fn generate_random(
        &self,
        id: &str,
        name: &str,
        starting_node: &str,
        tokens: usize,
    ) -> Result<(), String> {
        self.submit_job(id, name, starting_node, 0, tokens, false)
    }

    pub fn generate_with_seeded(
        &self,
        id: &str,
        name: &str,
        starting_node: &str,
        seed: u64,
        tokens: usize,
    ) -> Result<(), String> {
        self.submit_job(id, name, starting_node, seed, tokens, false)
    }

    pub fn generate_complete(
        &self,
        id: &str,
        name: &str,
        starting_node: &str,
        tokens: usize,
    ) -> Result<(), String> {
        self.submit_job(id, name, starting_node, 0, tokens, true)
    }

    pub fn generate_complete_with_seeded(
        &self,
        id: &str,
        name: &str,
        starting_node: &str,
        seed: u64,
        tokens: usize,
    ) -> Result<(), String> {
        self.submit_job(id, name, starting_node, seed, tokens, true)
    }

    /// Boots up the worker threads.
    pub fn start_resrap(&mut self) {
        if self.started {
            return;
        }
        self.started = true;

        let receivers = match self.pending_rx.take() {
            Some(receivers) => receivers,
            None => return,
        };
        let result_tx = match self.result_tx.clone() {
            Some(tx) => tx,
            None => return,
        };

        for rx in receivers {
            let result_tx = result_tx.clone();
            self.workers.push(std::thread::spawn(move || {
                while let Ok(job) = rx.recv() {
                    let code = match (job.graph.as_ref(), job.start_id) {
                        (Some(graph), Some(start_id)) => {
                            let prng = PRNG::new(job.seed);
                            if job.complete {
                                graph.walk_graph_id(prng, start_id, job.tokens, true)
                            } else {
                                graph.walk_graph_id(prng, start_id, job.tokens, false)
                            }
                        }
                        // Unknown grammar or start rule: return an empty result
                        // with the caller's id, matching the Go implementation.
                        _ => String::new(),
                    };

                    if result_tx.send(CodeGenRes { code, id: job.id }).is_err() {
                        break;
                    }
                }
            }));
        }
    }

    /// Gracefully ends the worker threads. Safe to call more than once.
    /// Pending results are drained to the code channel, which is then closed.
    pub fn shutdown_resrap(&mut self) {
        if self.closed {
            return;
        }
        self.closed = true;

        // Close the job queues; workers finish their current job and drain the
        // jobs already queued for them.
        self.pending_tx.clear();
        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }

        // Dropping the last worker sender closes the result channel after all
        // buffered results have been delivered.
        self.result_tx.take();
        self.pending_rx.take();
    }

    /// Alias using the Go method's capitalisation.
    pub fn shut_down_resrap(&mut self) {
        self.shutdown_resrap();
    }
}

impl crate::GrammarLoader for ResrapMT {
    fn parse_grammar(&mut self, name: &str, grammar: &str) -> Result<(), String> {
        self.parse_grammar_impl(name, grammar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    const GRAMMAR: &str = "program:function^;\nfunction:'f' '(' ')';\n";

    fn instance() -> ResrapMT {
        let mut r = ResrapMT::new(2, 10);
        r.parse_grammar("Test", GRAMMAR).unwrap();
        r.start_resrap();
        r
    }

    #[test]
    fn clamps_pool_and_queue() {
        let r = ResrapMT::new(0, 0);
        assert_eq!(r.pool_size(), 1);
        assert_eq!(r.wait_queue_size(), 1);
    }

    #[test]
    fn generates_a_job_with_its_id() {
        let mut r = instance();
        r.generate_random("job1", "Test", "program", 20).unwrap();
        let res = r
            .get_code_channel()
            .recv_timeout(Duration::from_secs(5))
            .unwrap();
        assert_eq!(res.id, "job1");
        assert!(!res.code.is_empty());
        r.shutdown_resrap();
    }

    #[test]
    fn handles_multiple_jobs() {
        let mut r = instance();
        for i in 0..10 {
            r.generate_random(&format!("job{}", i), "Test", "program", 10)
                .unwrap();
        }
        let channel = r.get_code_channel();
        let mut seen = std::collections::HashSet::new();
        for _ in 0..10 {
            let res = channel.recv_timeout(Duration::from_secs(5)).unwrap();
            assert!(!res.code.is_empty());
            seen.insert(res.id);
        }
        assert_eq!(seen.len(), 10);
        r.shutdown_resrap();
    }

    #[test]
    fn seeded_generation_is_deterministic_across_instances() {
        let mut a = instance();
        let mut b = instance();
        a.generate_with_seeded("1", "Test", "program", 999, 20)
            .unwrap();
        b.generate_with_seeded("1", "Test", "program", 999, 20)
            .unwrap();
        let ca = a
            .get_code_channel()
            .recv_timeout(Duration::from_secs(5))
            .unwrap();
        let cb = b
            .get_code_channel()
            .recv_timeout(Duration::from_secs(5))
            .unwrap();
        assert_eq!(ca.code, cb.code);
        a.shutdown_resrap();
        b.shutdown_resrap();
    }

    #[test]
    fn unknown_language_returns_empty_result() {
        let mut r = instance();
        r.generate_random("job", "Missing", "program", 10).unwrap();
        let res = r
            .get_code_channel()
            .recv_timeout(Duration::from_secs(5))
            .unwrap();
        assert_eq!(res.code, "");
        assert_eq!(res.id, "job");
        r.shutdown_resrap();
    }

    #[test]
    fn submit_after_shutdown_errors() {
        let mut r = instance();
        r.shutdown_resrap();
        assert_eq!(
            r.generate_random("job", "Test", "program", 10).unwrap_err(),
            ERR_SHUT_DOWN
        );
        r.shutdown_resrap();
    }

    #[test]
    fn supports_complete_and_words() {
        let mut r = instance();
        r.add_words("Test", "function", &["alpha".to_string()])
            .unwrap();
        r.generate_complete_with_seeded("job", "Test", "program", 7, 5)
            .unwrap();
        let res = r
            .get_code_channel()
            .recv_timeout(Duration::from_secs(5))
            .unwrap();
        assert!(!res.code.is_empty());
        assert!(res.code.starts_with("alpha"));
        assert!(!res.code.contains('f'));
        assert!(!res.code.contains('('));
        r.shutdown_resrap();
    }
}
