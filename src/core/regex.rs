use std::collections::HashMap;

use crate::core::prng::PRNG;

#[derive(Debug, Clone, Default)]
pub struct CacheRexState {
    cumu_freq: Vec<f32>,
    options: Vec<char>,
}

#[derive(Debug, Clone, Default)]
pub struct Regexer {
    pub cached_rex: HashMap<String, CacheRexState>,
}

impl Regexer {
    pub fn new() -> Self {
        Regexer {
            cached_rex: HashMap::new(),
        }
    }

    pub fn generate_string(&self, regex: &str, prn: &mut PRNG) -> String {
        let state = match self.cached_rex.get(regex) {
            Some(state) if !state.cumu_freq.is_empty() && !state.options.is_empty() => state,
            _ => return String::new(),
        };

        let size = prn.random_int(3, 8); // 3..=7 characters
        let mut result = String::with_capacity(size);

        for _ in 0..size {
            let x = prn.random();
            let idx = closest_index(&state.cumu_freq, x as f32);
            result.push(state.options[idx]);
        }

        result
    }

    pub fn cache_regex(&mut self, regex: &str) {
        let tokens = self.expand_class(regex);
        let mut bias_arr: Vec<f32> = Vec::with_capacity(tokens.len());
        let mut sum: f32 = 0.0;
        for token in &tokens {
            let bias = self.bias(*token) as f32;
            bias_arr.push(bias);
            sum += bias;
        }

        if sum > 0.0 {
            for b in &mut bias_arr {
                *b /= sum;
            }
        }

        let mut cdf: Vec<f32> = Vec::with_capacity(bias_arr.len());
        let mut cum = 0.0;
        for &w in &bias_arr {
            cum += w;
            cdf.push(cum);
        }

        self.cached_rex.insert(
            regex.to_string(),
            CacheRexState {
                cumu_freq: cdf,
                options: tokens,
            },
        );
    }

    pub fn expand_class(&self, class: &str) -> Vec<char> {
        let runes: Vec<char> = class.chars().collect();
        let mut chars = Vec::new();
        let mut i = 0;

        while i < runes.len() {
            if runes[i] == '\\' && i + 1 < runes.len() {
                chars.push(decode_class_escape(runes[i + 1]));
                i += 2;
                continue;
            }
            if i + 2 < runes.len() && runes[i + 1] == '-' && runes[i + 2] != '\\' {
                for c in runes[i]..=runes[i + 2] {
                    chars.push(c);
                }
                i += 3;
            } else {
                chars.push(runes[i]);
                i += 1;
            }
        }

        chars
    }

    /// Returns an integer weight for a rune based on usage frequency.
    pub fn bias(&self, c: char) -> i32 {
        if c.is_uppercase() {
            let mut b = self.bias(c.to_lowercase().next().unwrap_or(c)) / 2;
            if b < 1 {
                b = 1;
            }
            return b;
        }

        match c {
            'e' => 12,
            'a' | 'i' | 'o' => 9,
            'n' | 'r' | 't' | 's' | 'l' => 6,
            'c' | 'd' | 'm' | 'u' | 'p' | 'b' | 'g' => 4,
            'f' | 'h' | 'v' | 'k' | 'w' | 'y' => 3,
            'j' | 'x' | 'q' | 'z' => 1,
            _ => {
                if c.is_numeric() {
                    3
                } else if c == '_' {
                    5
                } else {
                    1
                }
            }
        }
    }
}

fn decode_class_escape(r: char) -> char {
    match r {
        'n' => '\n',
        't' => '\t',
        'r' => '\r',
        _ => r,
    }
}

fn closest_index(cdf: &[f32], x: f32) -> usize {
    for (i, &val) in cdf.iter().enumerate() {
        if x <= val {
            return i;
        }
    }
    cdf.len().saturating_sub(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_ranges_and_escapes() {
        let r = Regexer::new();
        assert_eq!(r.expand_class("a-c"), vec!['a', 'b', 'c']);
        assert_eq!(r.expand_class(r"\t\-"), vec!['\t', '-']);
        assert_eq!(r.expand_class("0-2_"), vec!['0', '1', '2', '_']);
    }

    #[test]
    fn uppercase_bias_is_halved() {
        let r = Regexer::new();
        assert_eq!(r.bias('e'), 12);
        assert_eq!(r.bias('E'), 6);
        assert_eq!(r.bias('_'), 5);
        assert_eq!(r.bias('0'), 3);
    }

    #[test]
    fn generates_three_to_seven_chars() {
        let mut r = Regexer::new();
        r.cache_regex("a-z");
        let mut p = PRNG::new(7);
        for _ in 0..100 {
            let s = r.generate_string("a-z", &mut p);
            assert!((3..8).contains(&s.chars().count()));
        }
    }

    #[test]
    fn unknown_regex_is_empty() {
        let r = Regexer::new();
        let mut p = PRNG::new(1);
        assert_eq!(r.generate_string("missing", &mut p), "");
    }

    #[test]
    fn empty_class_does_not_panic() {
        let mut r = Regexer::new();
        r.cache_regex("");
        let mut p = PRNG::new(1);
        assert_eq!(r.generate_string("", &mut p), "");
    }
}
