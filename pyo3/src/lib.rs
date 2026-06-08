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

use bikkurirs::ngram::NGramSurprisal;
use pyo3::prelude::*;
use pyo3::{
    Bound, PyResult,
    types::{PyAny, PyAnyMethods, PyModule, PyModuleMethods},
};

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

/// Python NGramSurprisal
#[pyclass(name = "NGramSurprisal")]
struct NGramSurprisalPy {
    ngramsurprisal: NGramSurprisal,
}

#[pymethods]
impl NGramSurprisalPy {
    #[new]
    fn __new__(n: i64) -> Self {
        NGramSurprisalPy {
            ngramsurprisal: NGramSurprisal::new(n),
        }
    }

    ///Calculate the underlying frequencies for the later surprisal calculations.
    ///Multiple calls will update the frequencies.
    #[pyo3(signature = (texts: "Iterable[Iterable[str]]") -> "None")]
    pub fn fit(&mut self, texts: &Bound<'_, PyAny>) -> PyResult<()> {
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
        self.ngramsurprisal.fit(&vec_texts);
        Ok(())
    }

    ///Calculate the surprisal for each word in each given text.
    #[pyo3(signature = (texts: "Iterable[Iterable[str]]") -> "list[list[tuple[str, float]]] | None")]
    pub fn surprisal(&self, texts: &Bound<'_, PyAny>) -> PyResult<Option<Vec<Vec<(String, f64)>>>> {
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
        Ok(self.ngramsurprisal.surprisal(&vec_texts))
    }
}

#[pyo3::pymodule]
pub mod bikkuri {
    use super::built_info;
    use super::register_submodules;
    use pyo3::prelude::*;

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

#[pyo3::pymodule(submodule, name = "ngram")]
pub mod ngram {
    #[pymodule_export]
    use super::NGramSurprisalPy;
}
