#!/usr/bin/env python3
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
"""Tokenizer tests for bikkuri."""

import unittest

from bikkuri import tokenizers


class NGramSurprisalTests(unittest.TestCase):
    """TopicContextModel tests."""

    def setUp(self) -> None:
        """Set up."""
        self.text = (
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod "
            + "tempor incididunt ut labore et dolore magna aliqua. Dignissim dictum "
            + "consequat accumsan risus volutpat laoreet. Platea ultrices sem "
            + "pellentesque rutrum orci turpis facilisis pharetra cubilia."
        )
        self.text2 = "Today is the 2nd and at 4pm the 3 of us meet."

    def test_regex_tokenizer(self) -> None:
        """Test unigram surprisal."""
        self.assertEqual(
            [
                "lorem",
                "ipsum",
                "dolor",
                "sit",
                "amet",
                "consectetur",
                "adipiscing",
                "elit",
                "sed",
                "do",
                "eiusmod",
                "tempor",
                "incididunt",
                "ut",
                "labore",
                "et",
                "dolore",
                "magna",
                "aliqua",
                "dignissim",
                "dictum",
                "consequat",
                "accumsan",
                "risus",
                "volutpat",
                "laoreet",
                "platea",
                "ultrices",
                "sem",
                "pellentesque",
                "rutrum",
                "orci",
                "turpis",
                "facilisis",
                "pharetra",
                "cubilia",
            ],
            tokenizers.regex(self.text),
        )
        self.assertEqual(
            [
                "today",
                "is",
                "the",
                "NUMnd",
                "and",
                "at",
                "NUMpm",
                "the",
                "NUM",
                "of",
                "us",
                "meet",
            ],
            tokenizers.regex(self.text2),
        )
        self.assertEqual(
            [
                "Lorem",
                "ipsum",
                "dolor",
                "sit",
                "amet",
                "consectetur",
                "adipiscing",
                "elit",
                "sed",
                "do",
                "eiusmod",
                "tempor",
                "incididunt",
                "ut",
                "labore",
                "et",
                "dolore",
                "magna",
                "aliqua",
                "Dignissim",
                "dictum",
                "consequat",
                "accumsan",
                "risus",
                "volutpat",
                "laoreet",
                "Platea",
                "ultrices",
                "sem",
                "pellentesque",
                "rutrum",
                "orci",
                "turpis",
                "facilisis",
                "pharetra",
                "cubilia",
            ],
            tokenizers.regex(self.text, to_lower=False),
        )
        self.assertEqual(
            [
                "Today",
                "is",
                "the",
                "NUMnd",
                "and",
                "at",
                "NUMpm",
                "the",
                "NUM",
                "of",
                "us",
                "meet",
            ],
            tokenizers.regex(self.text2, to_lower=False),
        )
        self.assertEqual(
            [
                "today",
                "is",
                "the",
                "2nd",
                "and",
                "at",
                "4pm",
                "the",
                "3",
                "of",
                "us",
                "meet",
            ],
            tokenizers.regex(self.text2, use_num_token=False),
        )


if __name__ == "__main__":
    unittest.main()
