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

use crate::errors::LoadError;

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

/// Calculates the surprisal of words, using Shannon Information, based on n-gram frequency.
#[derive(Clone, Debug)]
pub struct NGramFrequencySurprisal {
    pub n: i64,
    counts: HashMap<String, i64>,
    suffix_counts: HashMap<String, HashMap<String, i64>>,
}

impl NGramFrequencySurprisal {
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
        for (i, word) in (0_i64..).zip(text) {
            if i == suffix {
                break;
            } else {
                parts.pop_front();
                parts.push_back(word.as_ref().to_string());
            }
        }

        Some(parts.into_iter().collect::<Vec<_>>().join("_"))
    }

    pub fn new(n: i64) -> NGramFrequencySurprisal {
        NGramFrequencySurprisal {
            n,
            counts: HashMap::new(),
            suffix_counts: HashMap::new(),
        }
    }

    /// Load from a JSON-file.
    pub fn load(path: &Path) -> Result<NGramFrequencySurprisal, LoadError> {
        let mut s = String::new();
        {
            let mut file = File::open(path)?;
            file.read_to_string(&mut s)?;
        }
        let data = json::parse(&s)?;

        if data.is_object() {
            if !data.has_key("n") {
                return Err(LoadError::MissingKeyError("n".to_string()));
            } else if !data.has_key("counts") {
                return Err(LoadError::MissingKeyError("counts".to_string()));
            } else if !data.has_key("suffix_counts") {
                return Err(LoadError::MissingKeyError("suffix_counts".to_string()));
            }
        } else {
            return Err(LoadError::ParseError);
        }

        let n: i64 = data["n"].as_i64().unwrap();
        let mut counts: HashMap<String, i64> = HashMap::new();
        for (k, v) in data["counts"].entries() {
            counts.insert(k.to_string(), v.as_i64().unwrap());
        }

        let mut suffix_counts: HashMap<String, HashMap<String, i64>> = HashMap::new();
        for (k, v) in data["suffix_counts"].entries() {
            if n == 1 {
                return Err(LoadError::ParseError);
            }
            let mut scounts: HashMap<String, i64> = HashMap::new();
            for (k2, v2) in v.entries() {
                scounts.insert(k2.to_string(), v2.as_i64().unwrap());
            }
            suffix_counts.insert(k.to_string(), scounts);
        }

        Ok(NGramFrequencySurprisal {
            n,
            counts,
            suffix_counts,
        })
    }

    /// Calculate the underlying frequencies for the later surprisal calculations.
    /// Multiple calls will update the frequencies.
    pub fn fit<T, U, S: AsRef<str>>(&mut self, texts: &T)
    where
        for<'a> &'a T: IntoIterator<Item = &'a U>,
        for<'a> &'a U: IntoIterator<Item = &'a S>,
    {
        for text in texts {
            for (i, word) in (0_i64..).zip(text) {
                self.counts
                    .entry(word.as_ref().to_string())
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
                if self.n > 1 {
                    let prefix: String = self.get_prefix(text, i).unwrap();
                    if let Some(x) = self.suffix_counts.get_mut(word.as_ref()) {
                        x.entry(prefix)
                            .and_modify(|counter| *counter += 1)
                            .or_insert(1);
                    } else {
                        let mut suffix = HashMap::new();
                        suffix.insert(prefix, 1);
                        self.suffix_counts.insert(word.as_ref().to_string(), suffix);
                    }
                }
            }
        }
    }

    /// Save to a JSON-file.
    pub fn save(&self, path: &Path) -> io::Result<()> {
        let mut f = File::create(path)?;
        let mut data = json::JsonValue::new_object();
        data["n"] = self.n.into();
        data["counts"] = json::JsonValue::new_object();
        data["suffix_counts"] = json::JsonValue::new_object();

        let mut keys: Vec<String> = self.counts.clone().into_keys().collect();
        keys.sort_unstable();
        for key in keys {
            data["counts"][key] = self.counts[&key].into();
        }

        keys = self.suffix_counts.clone().into_keys().collect();
        keys.sort_unstable();
        for key in keys {
            data["suffix_counts"][key.clone()] = json::JsonValue::new_object();
            let mut skeys: Vec<String> = self.suffix_counts[&key].clone().into_keys().collect();
            skeys.sort_unstable();
            for skey in skeys {
                data["suffix_counts"][key.clone()][skey] = self.suffix_counts[&key][&skey].into();
            }
        }

        f.write_all(json::stringify(data).as_bytes())?;
        Ok(())
    }

    /// Calculate the surprisal for each word in each given text.
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
            let mut surprisal_text = vec![];
            for (i, word) in (0_i64..).zip(text) {
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
            }
            surprisal_texts.push(surprisal_text);
        }

        Some(surprisal_texts)
    }
}

impl PartialEq for NGramFrequencySurprisal {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
            && self.counts == other.counts
            && self.suffix_counts == other.suffix_counts
    }
}
