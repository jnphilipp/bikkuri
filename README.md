# Bikkuri

Calculate the surprisal of words in texts.

![Tests](https://github.com/jnphilipp/bikkuri/actions/workflows/tests.yml/badge.svg)
[![PyPI Version](https://img.shields.io/pypi/v/bikkuri.svg?logo=pypi&logoColor=white)](https://pypi.org/project/bikkuri/)
[![crates.io Version](https://img.shields.io/crates/v/bikkuri.svg?logo=rust&logoColor=white)](https://crates.io/crates/bikkuri)

## Usage

### Python
```python
from bikkuri.ngram import NGramFrequencySurprisal


unigram_surprisal = NGramFrequencySurprisal(1)
unigram_surprisal.fit([
    ["lorem", "ipsum", "dolor", "sit", "amet", ...],
    ["convallis", "fringilla", "dignissim", "massa", ...],
    ...
])

unigram_surprisal.surprisal([["lorem", "ipsum", "dolor"]])

unigram_surirsal.save("unigram_surprusal.json")
unigram_surirsal2 = NGramFrequencySurprisal.load("unigram_surprusal.json")
```

### Rust
```rust
extern crate bikkuri;
use bikkuri::ngram::NGramFrequencySurprisal;

let mut unigram_surprisal = NGramFrequencySurprisal::new(1);
unigram_surprisal.fit(&vec![
    vec!["lorem", "ipsum", "dolor", "sit", "amet", ...],
    vec!["convallis", "fringilla", "dignissim", "massa", ...],
    ...
]);
unigram_surprisal.surprisal(&vec![vec!["lorem", "ipsum", "dolor"]]);

let tmp_dir = TempDir::new("unigram_test")?;
let path = tmp_dir.path().join("unigram_surprisal.json");
unigram_surprisal.save(&path)?;

let mut unigram_surprisal2 = NGramFrequencySurprisal::load(&path)?;
```
