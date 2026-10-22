use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex, RwLock,
        mpsc::{self, Receiver, RecvError, RecvTimeoutError, Sender, SyncSender, TryRecvError},
    },
    thread::JoinHandle,
    time::Duration,
};

use crate::core::{file::Lang, prng::PRNG};

pub const ERR_SHUT_DOWN: &str = "resrap: instance has been shut down";

/// The result of one generation job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeGenRes {
    pub code: String,
    pub id: String,
}

/// A cloneable, blocking result channel. Multiple consumers may call `recv`;
/// disconnected means `ShutDownResrap` has finished draining every result.
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
    name: String,
    startnode: String,
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
    pending_tx: Option<SyncSender<CodeGenReq>>,
    pending_rx: Option<Arc<Mutex<Receiver<CodeGenReq>>>>,
    enqueue_tx: Option<Sender<CodeGenRes>>,
    code_channel: CodeChannel,
    dispatcher: Option<JoinHandle<()>>,
    workers: Vec<JoinHandle<()>>,
    started: bool,
    closed: bool,
}

impl ResrapMT {
    /// Creates a new multithreaded instance. `pool_size` and `wait_queue_size`
    /// are clamped to at least one, matching the reference implementation.
    pub fn new(pool_size: usize, wait_queue_size: usize) -> Self {
        let pool_size = pool_size.max(1);
        let wait_queue_size = wait_queue_size.max(1);

        let (pending_tx, pending_rx) = mpsc::sync_channel(wait_queue_size);
        let (enqueue_tx, enqueue_rx) = mpsc::channel::<CodeGenRes>();
        let (code_tx, code_rx) = mpsc::channel::<CodeGenRes>();

        // Results are queued in an unbounded internal buffer so workers never
        // block when the consumer is slower than the producer.
        let dispatcher = std::thread::spawn(move || {
            while let Ok(result) = enqueue_rx.recv() {
                if code_tx.send(result).is_err() {
                    break;
                }
            }
        });

        ResrapMT {
            language_graph: Arc::new(RwLock::new(HashMap::new())),
            pool_size,
            wait_queue_size,
            pending_tx: Some(pending_tx),
            pending_rx: Some(Arc::new(Mutex::new(pending_rx))),
            enqueue_tx: Some(enqueue_tx),
            code_channel: CodeChannel {
                rx: Arc::new(Mutex::new(code_rx)),
            },
            dispatcher: Some(dispatcher),
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
        if self.closed {
            return Err(ERR_SHUT_DOWN.to_string());
        }
        let tx = self.pending_tx.as_ref().ok_or(ERR_SHUT_DOWN)?;
        tx.send(req).map_err(|_| ERR_SHUT_DOWN.to_string())
    }

    pub fn generate_random(
        &self,
        id: &str,
        name: &str,
        starting_node: &str,
        tokens: usize,
    ) -> Result<(), String> {
        self.submit(CodeGenReq {
            name: name.to_string(),
            startnode: starting_node.to_string(),
            tokens,
            seed: 0,
            complete: false,
            id: id.to_string(),
        })
    }

    pub fn generate_with_seeded(
        &self,
        id: &str,
        name: &str,
        starting_node: &str,
        seed: u64,
        tokens: usize,
    ) -> Result<(), String> {
        self.submit(CodeGenReq {
            name: name.to_string(),
            startnode: starting_node.to_string(),
            tokens,
            seed,
            complete: false,
            id: id.to_string(),
        })
    }

    pub fn generate_complete(
        &self,
        id: &str,
        name: &str,
        starting_node: &str,
        tokens: usize,
    ) -> Result<(), String> {
        self.submit(CodeGenReq {
            name: name.to_string(),
            startnode: starting_node.to_string(),
            tokens,
            seed: 0,
            complete: true,
            id: id.to_string(),
        })
    }

    pub fn generate_complete_with_seeded(
        &self,
        id: &str,
        name: &str,
        starting_node: &str,
        seed: u64,
        tokens: usize,
    ) -> Result<(), String> {
        self.submit(CodeGenReq {
            name: name.to_string(),
            startnode: starting_node.to_string(),
            tokens,
            seed,
            complete: true,
            id: id.to_string(),
        })
    }

    /// Boots up the worker threads.
    pub fn start_resrap(&mut self) {
        if self.started {
            return;
        }
        self.started = true;

        let rx = match self.pending_rx.take() {
            Some(rx) => rx,
            None => return,
        };
        let enqueue_tx = match self.enqueue_tx.clone() {
            Some(tx) => tx,
            None => return,
        };

        for _ in 0..self.pool_size {
            let rx = Arc::clone(&rx);
            let enqueue_tx = enqueue_tx.clone();
            let language_graph = Arc::clone(&self.language_graph);
            self.workers.push(std::thread::spawn(move || {
                loop {
                    let job = {
                        let guard = rx.lock().unwrap();
                        guard.recv()
                    };
                    let job = match job {
                        Ok(job) => job,
                        Err(_) => break,
                    };

                    let code = {
                        let graph = language_graph.read().unwrap();
                        match graph.get(&job.name).and_then(|lang| lang.get_graph()) {
                            Some(graph) => {
                                let prng = PRNG::new(job.seed);
                                if job.complete {
                                    graph.walk_graph_complete(prng, &job.startnode, job.tokens)
                                } else {
                                    graph.walk_graph(prng, &job.startnode, job.tokens)
                                }
                            }
                            None => String::new(),
                        }
                    };

                    if enqueue_tx.send(CodeGenRes { code, id: job.id }).is_err() {
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

        // Close the job queue; workers finish their current job and drain it.
        self.pending_tx.take();
        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }

        // Dropping the last worker-side sender closes the dispatcher queue.
        self.enqueue_tx.take();
        if let Some(dispatcher) = self.dispatcher.take() {
            let _ = dispatcher.join();
        }

        // If the pool was never started, release the unused receiver.
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
