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

use json::Error as JsonError;
use std::error::Error;
use std::fmt;
use std::io::Error as IoError;

/// LoadError
#[derive(Debug)]
pub enum LoadError {
    IOError(IoError),
    JsonError(JsonError),
    MissingKeyError(String),
    ParseError,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoadError::IOError(e) => e.fmt(f),
            LoadError::JsonError(e) => e.fmt(f),
            LoadError::MissingKeyError(key) => write!(f, "Missing value for {key}."),
            LoadError::ParseError => write!(f, "Cannot parse file."),
        }
    }
}

impl Error for LoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            LoadError::IOError(ref e) => Some(e),
            LoadError::JsonError(ref e) => Some(e),
            LoadError::MissingKeyError(_) => None,
            LoadError::ParseError => None,
        }
    }
}

impl From<IoError> for LoadError {
    fn from(err: IoError) -> LoadError {
        LoadError::IOError(err)
    }
}

impl From<JsonError> for LoadError {
    fn from(err: JsonError) -> LoadError {
        LoadError::JsonError(err)
    }
}
