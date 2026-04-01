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
"""Bikkuri tokenizer module."""

import re

from typing import List


def regex(
    text: str,
    regex: str = r"\b(\w+(-\w+)+|\w+&\w+|\w+)\b",
    to_lower: bool = True,
    use_num_token: bool = True,
) -> List[str]:
    """Tokenize a text using a regex.

    Args:
     * text: text to tokenize
     * regex: regex to tokenize by, default to simple word boundaries

    Returns:
     * text as a list of tokens
    """
    words = []
    text = re.sub(r"https?://[^\s]+", "URL", text)
    if use_num_token:
        text = re.sub(r"-?\d[\d.,]*", "NUM", text)
    for m in re.finditer(re.compile(regex), text):
        if to_lower and m.group() != "URL" and "NUM" not in m.group():
            words.append(m.group().lower())
        else:
            words.append(m.group())
    words = [
        word
        for i, word in enumerate(words)
        if i == 0 or (word == "NUM" and words[i - 1] != word) or word != "NUM"
    ]
    return words
