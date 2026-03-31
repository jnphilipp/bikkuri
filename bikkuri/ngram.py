# Copyright (C) 2026 J. Nathanael Philipp (jnphilipp) <jnathanael@philipp.land>
#
# Bikkuri, calculate the surprisal of words in texts.
#
# This file is part of bikkuri.
#
# bikkuri is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# bikkuri is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with bikkuri. If not, see <http://www.gnu.org/licenses/>
"""N-gram surprisal calculations."""

import itertools
import math

from collections import Counter


class NGramSurprisal:
    """N-gram surprisal."""

    def __init__(self, n: int) -> None:
        """Init."""
        self._n = n
        self.__counter: Counter | None = None
        self.__suffix_counter: dict[str, Counter] | None = None

    def __call__(self, texts: list[list[str]]) -> list[list[tuple[str, float]]]:
        """Calculate the surprisal for the given texts."""
        return self.surprisal(texts)

    def _get_prefix(self, text: list[str], suffix: int) -> str | None:
        """Get the n-gram prefix of a word (suffix) in a text."""
        if self._n == 1:
            return None
        return "_".join(
            text[suffix - j] if suffix - j >= 0 else "_" for j in range(1, self._n)
        )

    def fit(self, texts: list[list[str]]) -> None:
        """Calculate the frequencies needed to calculate the n-gram surprisal."""
        if self.__counter is None:
            self.__counter = Counter()
        if self.__suffix_counter is None:
            self.__suffix_counter = {}

        for text in texts:
            for i, word in enumerate(text):
                self.__counter.update([word])
                if self._n > 1:
                    if word not in self.__suffix_counter:
                        self.__suffix_counter[word] = Counter()
                    self.__suffix_counter[word].update([self._get_prefix(text, i)])

    @property
    def n(self) -> int:
        """Get the type of n-gram."""
        return self._n

    def surprisal(self, texts: list[list[str]]) -> list[list[tuple[str, float]]]:
        """Calculate the surprisal for the given texts."""

        def get_prefix_count(prefix: str | None) -> int:
            assert self.__suffix_counter is not None

            if prefix is None:
                return 0

            return sum(
                v[prefix] for k, v in self.__suffix_counter.items() if prefix in v
            )

        def ngram_prob(text: list[str], word: str, idx: int) -> float:
            """Calculate the probability of a ngram."""
            assert self.__counter is not None
            assert self.__suffix_counter is not None

            prefix = self._get_prefix(text, idx)
            if word in self.__suffix_counter and prefix in self.__suffix_counter[word]:
                return self.__suffix_counter[word][prefix] / get_prefix_count(prefix)
            elif (
                word in self.__suffix_counter
                and prefix not in self.__suffix_counter[word]
            ):
                return self.__counter[word] / self.__counter.total()
            else:
                return 1.0 / (
                    get_prefix_count(prefix) + 1
                    if prefix
                    in itertools.chain(
                        *[
                            list(self.__suffix_counter[k].keys())
                            for k in self.__suffix_counter.keys()
                        ]
                    )
                    else self.__counter.total() + 1
                )

        assert self.__counter is not None

        if self._n == 1:
            return [
                [
                    (
                        word,
                        (
                            -1.0
                            * math.log2(
                                (self.__counter[word] / self.__counter.total())
                                if word in self.__counter
                                else (1.0 / (self.__counter.total() + 1.0))
                            )
                        ),
                    )
                    for word in text
                ]
                for text in texts
            ]
        else:
            return [
                [
                    (word, -1.0 * math.log2(ngram_prob(text, word, i)))
                    for i, word in enumerate(text)
                ]
                for text in texts
            ]


class UniGramSurprisal(NGramSurprisal):
    """Uni-gram surprisal."""

    def __init__(self) -> None:
        """Init."""
        super().__init__(1)


class BiGramSurprisal(NGramSurprisal):
    """Bi-gram surprisal."""

    def __init__(self) -> None:
        """Init."""
        super().__init__(2)
