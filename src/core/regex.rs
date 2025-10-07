use std::collections::HashMap;

use crate::core::prng::PRNG;

#[derive(Debug, Clone)]
struct CacheRexState {
    cumu_freq: Vec<f32>,
    options: Vec<char>,
}

#[derive(Debug)]
pub struct Regexer {
    cached_rex: HashMap<String, CacheRexState>,
}

impl Regexer {
    pub fn new() -> Self {
        Regexer {
            cached_rex: HashMap::new(),
        }
    }
    pub fn generate_string(&self, regex: &str, prn: &mut PRNG) -> String {
        let size = prn.random_int(3, 4); // generate size between 3 and 4 (you can adjust for 3-7)
        let mut result = String::with_capacity(size as usize);

        if let Some(state) = self.cached_rex.get(regex) {
            for _ in 0..size {
                let x = prn.random(); // float 0-1
                let idx = closest_index(&state.cumu_freq, x as f32);
                result.push(state.options[idx]);
            }
        }

        result
    }

    fn expand_class(&self, class: &str) -> Vec<char> {
        let runes: Vec<char> = class.chars().collect();
        let mut chars = Vec::new();
        let mut i = 0;

        while i < runes.len() {
            if i + 2 < runes.len() && runes[i + 1] == '-' {
                // range a-z
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

    pub fn cache_regex(&mut self, regex: &str) {
        let tokens = self.expand_class(regex);
        let mut bias_arr: Vec<f32> = Vec::with_capacity(tokens.len());
        let mut sum: f32 = 0.0;
        for token in &tokens {
            let bias = self.bias(token.clone()) as f32;
            bias_arr.push(bias);
            sum += bias;
        }
        for b in &mut bias_arr {
            *b /= sum;
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
    fn bias(&self, r: char) -> i32 {
        let r_lower = r.to_ascii_lowercase();

        match r_lower {
            'e' => 12,
            'a' | 'i' | 'o' => 9,
            'n' | 'r' | 't' | 's' | 'l' => 6,
            'c' | 'd' | 'm' | 'u' | 'p' | 'b' | 'g' => 4,
            'f' | 'h' | 'v' | 'k' | 'w' | 'y' => 3,
            'j' | 'x' | 'q' | 'z' => 1,
            _ => {
                if r.is_uppercase() {
                    self.bias(r.to_ascii_lowercase()) / 2
                } else if r.is_digit(10) {
                    3
                } else if r == '_' {
                    5
                } else {
                    1
                }
            }
        }
    }
}

fn closest_index(cdf: &[f32], x: f32) -> usize {
    for (i, &val) in cdf.iter().enumerate() {
        if x <= val {
            return i;
        }
    }
    cdf.len() - 1
}
