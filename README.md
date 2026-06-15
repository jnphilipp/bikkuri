# Bikkuri

Calculate the surprisal of words in texts.

![Tests](https://github.com/jnphilipp/bikkuri/actions/workflows/tests.yml/badge.svg)
[![PyPI Version](https://img.shields.io/pypi/v/bikkuri.svg?logo=pypi&logoColor=white)](https://pypi.org/project/bikkuri/)
[![crates.io Version](https://img.shields.io/crates/v/bikkuri.svg?logo=rust&logoColor=white)](https://crates.io/project/bikkuri/)

## Usage

### Python
```python
from bikkuri.ngram import NGramSurprisal


unigram_surprisal = NGramSurprisal(1)
unigram_surprisal.fit([
    ["lorem", "ipsum", "dolor", "sit", "amet", ...],
    ["convallis", "fringilla", "dignissim", "massa", ...],
    ...
])

unigram_surprisal.surprisal([["lorem", "ipsum", "dolor"]])
```

### Rust
```rust
extern crate bikkuri;
use bikkuri::ngram::NGramSurprisal;

let mut unigram_surprisal = NGramSurprisal::new(1);
unigram_surprisal.fit(&vec![
    vec!["lorem", "ipsum", "dolor", "sit", "amet", ...],
    vec!["convallis", "fringilla", "dignissim", "massa", ...],
    ...
]);
unigram_surprisal.surprisal(&vec![vec!["lorem", "ipsum", "dolor"]]);
```
