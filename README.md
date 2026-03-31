# Bikkuri

Calculate the surprisal of words in texts.

![Tests](https://github.com/jnphilipp/bikkuri/actions/workflows/tests.yml/badge.svg)
[![pypi Version](https://img.shields.io/pypi/v/bikkuri.svg?logo=pypi&logoColor=white)](https://pypi.org/project/bikkuri/)

## Requirements

* Python >= 3.11

## Usage

```python
from bikkuri import UniGramSurprisal


unigram_surprisal = UniGramSurprisal()
unigram_surprisal.fit([
    ["lorem", "ipsum", "dolor", "sit", "amet", ...],
    ["convallis", "fringilla", "dignissim", "massa", ...],
    ...
])

unigram_surprisal([["lorem", "ipsum", "dolor"]])
```
