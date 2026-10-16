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

use regex;
use std::error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

/// Error
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Message(String),
    MissingKey(String),
    ModelUntrained,
    ParseInt(ParseIntError),
    Regex(regex::Error),
    UnrecognisedFormat,
    WorkerPanicked,
    WorkerSend,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e) => e.fmt(f),
            Error::Message(msg) => write!(f, "{}", msg),
            Error::MissingKey(key) => write!(f, "Missing value for {key}."),
            Error::ModelUntrained => write!(f, "The model is untrained and cannot be used yet."),
            Error::ParseInt(e) => e.fmt(f),
            Error::Regex(e) => e.fmt(f),
            Error::UnrecognisedFormat => write!(f, "Unrecognised format."),
            Error::WorkerPanicked => write!(f, "A worker panicked."),
            Error::WorkerSend => write!(f, "One or more workers failed to send their results."),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::Message(_) => None,
            Error::MissingKey(_) => None,
            Error::ModelUntrained => None,
            Error::ParseInt(ref e) => Some(e),
            Error::Regex(ref e) => Some(e),
            Error::UnrecognisedFormat => None,
            Error::WorkerPanicked => None,
            Error::WorkerSend => None,
        }
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Error {
        Error::Regex(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Message(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
