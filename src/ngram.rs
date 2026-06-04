// Copyright (C) 2026 J. Nathanael Philipp (jnphilipp) <jnathanael@philipp.land>
//
// Bikkuri, calculate the surprisal of words in texts.
//
// This file is part of bikkuri.
//
// bikkuri is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// bikkuri is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with bikkuri. If not, see <http://www.gnu.org/licenses/>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::{HashMap, VecDeque};

/// NGramSurprisal
pub struct NGramSurprisal {
    pub n: i64,
    counts: HashMap<String, i64>,
    suffix_counts: HashMap<String, HashMap<String, i64>>,
}

impl NGramSurprisal {
    pub fn new(n: i64) -> NGramSurprisal {
        NGramSurprisal {
            n: n,
            counts: HashMap::new(),
            suffix_counts: HashMap::new(),
        }
    }

    fn get_prefix<T, S: AsRef<str>>(&self, text: &T, suffix: i64) -> Option<String>
    where
        for<'a> &'a T: IntoIterator<Item = &'a S>,
    {
        if self.n <= 1 {
            return None;
        }

        let mut parts: VecDeque<String> = VecDeque::new();
        for _ in 1..self.n {
            parts.push_back("_".to_string());
        }
        let mut i: i64 = 0;
        for word in text {
            if i == suffix {
                break;
            } else {
                parts.pop_front();
                parts.push_back(word.as_ref().to_string());
            }
            i += 1;
        }

        Some(parts.into_iter().collect::<Vec<_>>().join("_"))
    }

    pub fn fit<T, U, S: AsRef<str>>(&mut self, texts: &T)
    where
        for<'a> &'a T: IntoIterator<Item = &'a U>,
        for<'a> &'a U: IntoIterator<Item = &'a S>,
    {
        for text in texts {
            let mut i: i64 = 0;
            for word in text {
                self.counts
                    .entry(word.as_ref().to_string())
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
                if self.n > 1 {
                    let prefix: String = self.get_prefix(text, i).unwrap();
                    if let Some(x) = self.suffix_counts.get_mut(&word.as_ref().to_string()) {
                        x.entry(prefix)
                            .and_modify(|counter| *counter += 1)
                            .or_insert(1);
                    } else {
                        let mut suffix = HashMap::new();
                        suffix.insert(prefix, 1);
                        self.suffix_counts.insert(word.as_ref().to_string(), suffix);
                    }
                }
                i += 1;
            }
        }
    }

    pub fn surprisal<T, U, S: AsRef<str>>(&self, texts: &T) -> Option<Vec<Vec<(String, f64)>>>
    where
        for<'a> &'a T: IntoIterator<Item = &'a U>,
        for<'a> &'a U: IntoIterator<Item = &'a S>,
    {
        if self.counts.is_empty() {
            return None;
        }

        let prefix_count = |prefix: &String| -> f64 {
            self.suffix_counts
                .values()
                .map(|v| v.get(prefix).copied().unwrap_or(0) as f64)
                .sum()
        };

        let mut surprisal_texts = vec![];
        let total: f64 = self.counts.values().map(|&v| v as f64).sum();
        for text in texts {
            let mut i: i64 = 0;
            let mut surprisal_text = vec![];
            for word in text {
                let p: f64;
                let w = word.as_ref().to_string();
                if self.n == 1 {
                    if self.counts.contains_key(&w) {
                        p = *self.counts.get(&w).unwrap() as f64 / total;
                    } else {
                        p = 1.0 / (total + 1.0);
                    }
                } else {
                    let prefix: String = self.get_prefix(text, i).unwrap();
                    if self.suffix_counts.contains_key(&w)
                        && self.suffix_counts.get(&w).unwrap().contains_key(&prefix)
                    {
                        p = *self.suffix_counts.get(&w).unwrap().get(&prefix).unwrap() as f64
                            / prefix_count(&prefix);
                    } else if self.suffix_counts.contains_key(&w) {
                        p = *self.counts.get(&w).unwrap() as f64 / total;
                    } else if prefix_count(&prefix) == 0.0 {
                        p = 1.0 / (total + 1.0)
                    } else {
                        p = 1.0 / (prefix_count(&prefix) + 1.0);
                    }
                }
                surprisal_text.push((w, -p.log2()));
                i += 1;
            }
            surprisal_texts.push(surprisal_text);
        }

        Some(surprisal_texts)
    }
}
