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

use crate::errors::Error;

use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;
use std::sync::mpsc::{self, SendError};
use std::thread;

/// Calculates the surprisal of words, using Shannon Information, based on n-gram frequency.
#[derive(Clone, Debug)]
pub struct NGramFrequencySurprisal {
    pub n: i64,
    counts: HashMap<String, i64>,
    suffix_counts: HashMap<String, HashMap<String, i64>>,
}

impl NGramFrequencySurprisal {
    fn get_prefix<T, S: AsRef<str>>(text: &T, suffix: i64, n: i64) -> Option<String>
    where
        for<'a> &'a T: IntoIterator<Item = &'a S>,
    {
        if n <= 1 {
            return None;
        }

        let mut parts: VecDeque<String> = VecDeque::new();
        for _ in 1..n {
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
    pub fn load(path: &Path) -> Result<NGramFrequencySurprisal, Error> {
        let mut s = String::new();
        {
            let mut file = File::open(path)?;
            file.read_to_string(&mut s)?;
        }

        Ok(s.parse::<NGramFrequencySurprisal>()?)
    }

    /// Calculate the underlying frequencies for the later surprisal calculations.
    /// Multiple calls will update the frequencies.
    pub fn fit<T, U, S: AsRef<str>>(&mut self, texts: &T) -> Result<(), Error>
    where
        U: Sync,
        S: AsRef<str> + Sync,
        for<'a> &'a T: IntoIterator<Item = &'a U>,
        for<'a> &'a U: IntoIterator<Item = &'a S>,
    {
        let n = self.n;

        let texts: Vec<&U> = texts.into_iter().collect();

        thread::scope(|scope| -> Result<(), Error> {
            let (sender, receiver) =
                mpsc::channel::<(HashMap<String, i64>, HashMap<String, HashMap<String, i64>>)>();

            let handles: Vec<_> = texts
                .iter()
                .map(|text| {
                    let sender = sender.clone();
                    scope.spawn(move || -> Result<(), SendError<(HashMap<String, i64>, HashMap<String, HashMap<String, i64>>)>> {
                        let mut counts: HashMap<String, i64> = HashMap::new();
                        let mut suffix_counts: HashMap<String, HashMap<String, i64>> = HashMap::new();
                        for (i, word) in (0_i64..).zip(*text) {
                            *counts
                                .entry(word.as_ref().to_string())
                                .or_insert(0) += 1;

                            if n > 1 {
                                let prefix: String = Self::get_prefix(*text, i, n).unwrap();
                                *suffix_counts
                                    .entry(word.as_ref().to_string())
                                    .or_default()
                                    .entry(prefix)
                                    .or_insert(0) += 1;
                            }
                        }
                        sender.send((counts, suffix_counts))?;
                        Ok(())
                    })
                }).collect();

            drop(sender);

            for (counts, suffix_counts) in receiver {
                for (word, count) in counts {
                    *self.counts.entry(word).or_insert(0) += count;
                }

                for (word, prefixes) in suffix_counts {
                    let suffix = self.suffix_counts.entry(word).or_default();
                    for (prefix, count) in prefixes {
                        *suffix.entry(prefix).or_insert(0) += count;
                    }
                }
            }

            for handle in handles {
                match handle.join() {
                    Ok(Ok(())) => {}
                    Ok(Err(_send_error)) => {
                        return Err(Error::WorkerSend);
                    }
                    Err(_panic_payload) => {
                        return Err(Error::WorkerPanicked);
                    }
                }
            }

            Ok(())
        })
    }

    /// Save to a file.
    pub fn save(&self, path: &Path) -> io::Result<()> {
        write!(File::create(path)?, "{}", self)
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
                    let prefix: String =
                        NGramFrequencySurprisal::get_prefix(text, i, self.n).unwrap();
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

impl fmt::Display for NGramFrequencySurprisal {
    /// Display a n-gram frequency surprisal as a JSON str.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"n\":{},", self.n)?;

        write!(f, "\"counts\":{{")?;
        let mut keys: Vec<String> = self.counts.clone().into_keys().collect();
        keys.sort_unstable();
        let mut iter = keys.iter().peekable();
        while let Some(key) = iter.next() {
            write!(f, "\"{}\":{}", key, self.counts[key])?;
            if !iter.peek().is_none() {
                write!(f, ",")?;
            }
        }

        write!(f, "}},\"suffix_counts\":{{")?;
        keys = self.suffix_counts.clone().into_keys().collect();
        keys.sort_unstable();
        let mut iter = keys.iter().peekable();
        while let Some(key) = iter.next() {
            write!(f, "\"{}\":{{", key)?;

            let mut skeys: Vec<String> = self.suffix_counts[key].clone().into_keys().collect();
            skeys.sort_unstable();
            let mut siter = skeys.iter().peekable();
            while let Some(skey) = siter.next() {
                write!(f, "\"{}\":{}", skey, self.suffix_counts[key][skey])?;
                if !siter.peek().is_none() {
                    write!(f, ",")?;
                }
            }

            write!(f, "}}")?;
            if !iter.peek().is_none() {
                write!(f, ",")?;
            }
        }

        write!(f, "}}}}")?;
        Ok(())
    }
}

impl FromStr for NGramFrequencySurprisal {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r#"^\{"n":(?<n>\d+),\"counts\":(?<counts>\{.*\}),\"suffix_counts\":(?<suffix_counts>\{.*\})\}\s*$"#,
        )?;

        match re.captures(s) {
            Some(caps) => {
                let n: i64 = caps["n"].parse()?;
                let mut counts: HashMap<String, i64> = HashMap::new();
                let mut suffix_counts: HashMap<String, HashMap<String, i64>> = HashMap::new();

                let re = Regex::new(r#""(?<word>[^"]+)":(?<count>\d+)"#)?;
                for (_, [word, count]) in re.captures_iter(&caps["counts"]).map(|c| c.extract()) {
                    counts.insert(word.to_string(), count.parse::<i64>()?);
                }

                let re = Regex::new(r#""(?<word>[^"]+)":\{(?<suffixes>[^\}]*)\}"#)?;
                for (_, [word, suffixes]) in re
                    .captures_iter(&caps["suffix_counts"])
                    .map(|c| c.extract())
                {
                    if !counts.contains_key(word) {
                        return Err(Error::UnrecognisedFormat);
                    }

                    let mut scounts: HashMap<String, i64> = HashMap::new();
                    let re = Regex::new(r#""(?<suffix>[^"]+)":(?<count>\d+)"#)?;
                    for (_, [suffix, count]) in re.captures_iter(&suffixes).map(|c| c.extract()) {
                        scounts.insert(suffix.to_string(), count.parse::<i64>()?);
                    }
                    suffix_counts.insert(word.to_string(), scounts);
                }

                if n > 1
                    && (counts.len() != suffix_counts.len()
                        || !counts.keys().all(|k| suffix_counts.contains_key(k)))
                {
                    return Err(Error::UnrecognisedFormat);
                } else if n == 1 && suffix_counts.len() > 0 {
                    return Err(Error::UnrecognisedFormat);
                }

                Ok(NGramFrequencySurprisal {
                    n,
                    counts,
                    suffix_counts,
                })
            }
            None => {
                let re = Regex::new(r#"^\{"#)?;
                if re.is_match(s) {
                    let re = Regex::new(r#"^\{"n":"#)?;
                    if re.is_match(s) {
                        let re = Regex::new(r#"^\{"n":\d+,"counts":"#)?;
                        if re.is_match(s) {
                            let re = Regex::new(r#"^\{"n":\d+,"counts":.+?,"suffix_counts":"#)?;
                            if re.is_match(s) {
                                Err(Error::UnrecognisedFormat)
                            } else {
                                Err(Error::MissingKey("suffix_counts".to_string()))
                            }
                        } else {
                            Err(Error::MissingKey("counts".to_string()))
                        }
                    } else {
                        Err(Error::MissingKey("n".to_string()))
                    }
                } else {
                    Err(Error::UnrecognisedFormat)
                }
            }
        }
    }
}

impl PartialEq for NGramFrequencySurprisal {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
            && self.counts == other.counts
            && self.suffix_counts == other.suffix_counts
    }
}
