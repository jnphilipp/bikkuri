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

extern crate bikkuri as bikkurirs;
extern crate pyo3;
#[macro_use]
extern crate pyo3_built;

use bikkurirs::errors::Error;
use bikkurirs::ngram::NGramFrequencySurprisal;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::{
    Bound, PyErr, PyResult,
    types::{PyAny, PyAnyMethods, PyModule, PyModuleMethods},
};
use std::path::PathBuf;

//#[allow(dead_code)]
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn register_submodules(
    module: &Bound<'_, PyModule>,
    parent_name: &str,
    sys_modules: &Bound<'_, PyAny>,
) -> PyResult<()> {
    for attr_name in module.index()? {
        let attr_name: String = attr_name.extract()?;
        let attr = module.getattr(&attr_name)?;

        if let Ok(submodule) = attr.cast::<PyModule>() {
            let parent_name = format!("{}.{}", parent_name, attr_name);
            sys_modules.set_item(&parent_name, submodule)?;
            register_submodules(submodule, &parent_name, sys_modules)?;
        }
    }

    Ok(())
}

/// Error
#[pyclass(name = "Error")]
struct ErrorPy(Error);

impl From<Error> for ErrorPy {
    fn from(err: Error) -> Self {
        Self(err)
    }
}

impl std::convert::From<ErrorPy> for PyErr {
    fn from(err: ErrorPy) -> Self {
        match err {
            ErrorPy(Error::Io(ref e)) => exceptions::PyOSError::new_err(e.to_string()),
            ErrorPy(Error::Message(ref e)) => exceptions::PyRuntimeError::new_err(e.to_string()),
            ErrorPy(Error::MissingKey(ref e)) => exceptions::PyKeyError::new_err(e.to_string()),
            ErrorPy(Error::ModelUntrained) => exceptions::PyRuntimeError::new_err(
                "The model is untrained and cannot be used yet.".to_string(),
            ),
            ErrorPy(Error::ParseInt(ref e)) => exceptions::PyValueError::new_err(e.to_string()),
            ErrorPy(Error::Regex(ref e)) => exceptions::PySyntaxError::new_err(e.to_string()),
            ErrorPy(Error::UnrecognisedFormat) => {
                exceptions::PyValueError::new_err("Unrecognised format.".to_string())
            }
            ErrorPy(Error::WorkerPanicked) => {
                exceptions::PyRuntimeError::new_err("A worker panicked.".to_string())
            }
            ErrorPy(Error::WorkerSend) => exceptions::PyRuntimeError::new_err(
                "One or more workers failed to send their results.".to_string(),
            ),
            ErrorPy(Error::ZeroNGram) => {
                exceptions::PyRuntimeError::new_err("N-gram needs to be non zero.".to_string())
            }
        }
    }
}

impl From<PyErr> for ErrorPy {
    fn from(err: PyErr) -> Self {
        ErrorPy(Error::Message(err.to_string()))
    }
}

/// Calculates the surprisal of words, using Shannon Information, based on n-gram frequency.
#[pyclass(eq, name = "NGramFrequencySurprisal")]
#[derive(PartialEq)]
struct NGramFrequencySurprisalPy {
    model: NGramFrequencySurprisal,
}

#[pymethods]
impl NGramFrequencySurprisalPy {
    #[new]
    fn __new__(n: usize) -> Result<Self, ErrorPy> {
        Ok(NGramFrequencySurprisalPy {
            model: NGramFrequencySurprisal::new(n)?,
        })
    }

    pub fn __str__(&self) -> String {
        self.model.to_string()
    }

    /// Create from a JSON string.
    #[staticmethod]
    #[pyo3(signature = (s: "str") -> "NGramFrequencySurprisal")]
    fn from_str(s: String) -> Result<Self, ErrorPy> {
        Ok(NGramFrequencySurprisalPy {
            model: s.parse::<NGramFrequencySurprisal>()?,
        })
    }

    /// Load from a JSON file.
    #[staticmethod]
    #[pyo3(signature = (path: "Path") -> "NGramFrequencySurprisal")]
    fn load(path: PathBuf) -> Result<Self, ErrorPy> {
        Ok(NGramFrequencySurprisalPy {
            model: NGramFrequencySurprisal::load(path.as_path())?,
        })
    }

    /// Calculate the underlying frequencies for the later surprisal calculations.
    /// Multiple calls will update the frequencies.
    #[pyo3(signature = (texts: "Iterable[Iterable[str]]") -> "None")]
    pub fn fit(&mut self, texts: &Bound<'_, PyAny>) -> Result<(), ErrorPy> {
        let outer = texts.try_iter()?;
        let mut vec_texts = vec![];
        for outer_item in outer {
            let inner_any = outer_item?;
            let inner = inner_any.try_iter()?;
            let mut vec_text = vec![];
            for inner_item in inner {
                let s: String = inner_item?.extract()?;
                vec_text.push(s);
            }
            vec_texts.push(vec_text);
        }
        self.model.fit(&vec_texts)?;
        Ok(())
    }

    /// Save to a JSON-file.
    #[pyo3(signature = (path: "Path") -> "None")]
    pub fn save(&self, path: PathBuf) -> PyResult<()> {
        Ok(self.model.save(path.as_path())?)
    }

    /// Calculate the surprisal for each word in each given text.
    #[pyo3(signature = (texts: "Iterable[Iterable[str]]") -> "list[list[tuple[str, float]]] | None")]
    pub fn surprisal(&self, texts: &Bound<'_, PyAny>) -> Result<Vec<Vec<(String, f64)>>, ErrorPy> {
        let outer = texts.try_iter()?;
        let mut vec_texts = vec![];
        for outer_item in outer {
            let inner_any = outer_item?;
            let inner = inner_any.try_iter()?;
            let mut vec_text = vec![];
            for inner_item in inner {
                let s: String = inner_item?.extract()?;
                vec_text.push(s);
            }
            vec_texts.push(vec_text);
        }
        Ok(self.model.surprisal(&vec_texts)?)
    }
}

#[pyo3::pymodule]
pub mod bikkuri {
    use super::built_info;
    use super::register_submodules;
    use pyo3::prelude::*;

    #[pymodule_export]
    use super::errors;

    #[pymodule_export]
    use super::ngram;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        let py = m.py();
        m.add("__version__", env!("CARGO_PKG_VERSION"))?;
        m.add("__app_name__", "bikkuri")?;
        m.add("__author__", "J. Nathanael Philipp")?;
        m.add("__email__", "jnathanael@philipp.land")?;
        m.add(
            "__copyright__",
            "Copyright 2026 J. Nathanael Philipp (jnphilipp)",
        )?;
        m.add("__license__", "GPL-3.0-or-later")?;
        m.add("__repository__", "https://github.com/jnphilipp/bikkuri/")?;
        m.add(
            "__build__",
            pyo3_built!(py, built_info, "time", "git", "target"),
        )?;
        m.add(
            "VERSION",
            format!(
                "bikkuri v{}\nCopyright 2026 J. Nathanael Philipp (jnphilipp)\nLicense GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\nReport bugs to https://github.com/jnphilipp/bikkuri/issues.\nWritten by {}",
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_AUTHORS"),
            )
        )?;
        register_submodules(m, "bikkuri", &m.py().import("sys")?.getattr("modules")?)?;
        Ok(())
    }
}

#[pyo3::pymodule(submodule, name = "errors")]
pub mod errors {
    #[pymodule_export]
    use super::ErrorPy;
}

#[pyo3::pymodule(submodule, name = "ngram")]
pub mod ngram {
    #[pymodule_export]
    use super::NGramFrequencySurprisalPy;
}
