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

extern crate bikkuri;
extern crate tempdir;

use bikkuri::errors::LoadError;
use bikkuri::ngram::NGramFrequencySurprisal;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use tempdir::TempDir;

fn get_texts() -> Vec<Vec<&'static str>> {
    vec![
        vec![
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
            "lectus",
            "vitae",
            "mi",
            "mauris",
            "facilisis",
            "luctus",
            "posuere",
            "elementum",
            "cursus",
            "fermentum",
            "non",
            "massa",
            "litora",
            "ultricies",
            "donec",
            "consectetur",
            "lectus",
            "sodales",
            "elementum",
            "facilisis",
            "fermentum",
            "senectus",
            "et",
            "mi",
            "lobortis",
            "iaculis",
            "fermentum",
            "ante",
            "consequat",
            "aptent",
            "quam",
            "ultricies",
            "sed",
            "libero",
        ],
        vec![
            "convallis",
            "fringilla",
            "dignissim",
            "massa",
            "arcu",
            "posuere",
            "in",
            "imperdiet",
            "eu",
            "at",
            "aenean",
            "sit",
            "sollicitudin",
            "arcu",
            "consectetur",
            "bibendum",
            "a",
            "class",
            "consequat",
            "nulla",
            "fusce",
            "donec",
            "fringilla",
            "lobortis",
            "vehicula",
            "tristique",
            "hac",
            "vestibulum",
            "aenean",
            "fusce",
            "hac",
            "eros",
            "pulvinar",
            "cubilia",
            "ultricies",
            "bibendum",
            "tempor",
            "at",
            "gravida",
            "malesuada",
        ],
        vec![
            "quam",
            "nullam",
            "eget",
            "est",
            "ac",
            "fusce",
            "elit",
            "eros",
            "tellus",
            "facilisis",
            "malesuada",
            "hendrerit",
            "lectus",
            "dictumst",
            "praesent",
            "sodales",
            "augue",
            "netus",
            "nullam",
            "eget",
            "justo",
            "duis",
            "nulla",
            "eros",
            "potenti",
            "at",
        ],
        vec![
            "sagittis",
            "non",
            "nullam",
            "maximus",
            "purus",
            "nisi",
            "ante",
            "sollicitudin",
            "etiam",
            "urna",
            "volutpat",
            "mauris",
            "semper",
            "sollicitudin",
            "felis",
            "aliquet",
            "nullam",
            "praesent",
            "vel",
            "curabitur",
            "habitasse",
            "condimentum",
            "metus",
            "eu",
            "pretium",
            "maximus",
            "tristique",
            "litora",
            "nisl",
            "quam",
            "purus",
            "facilisis",
            "laoreet",
            "dignissim",
        ],
        vec![
            "dui",
            "proin",
            "adipiscing",
            "consequat",
            "felis",
            "amet",
            "laoreet",
            "blandit",
            "laoreet",
            "porttitor",
            "litora",
            "in",
            "orci",
            "quis",
            "himenaeos",
            "nam",
            "adipiscing",
            "nisi",
            "metus",
            "pulvinar",
            "ante",
            "dignissim",
            "dolor",
            "dapibus",
            "a",
            "vivamus",
            "nisi",
        ],
        vec![
            "sit",
            "dapibus",
            "neque",
            "sapien",
            "cubilia",
            "porttitor",
            "egestas",
            "proin",
            "nunc",
            "purus",
            "nisi",
            "dapibus",
            "nunc",
            "venenatis",
            "cras",
            "mauris",
            "suscipit",
            "pellentesque",
            "netus",
            "porta",
            "vitae",
            "fermentum",
            "turpis",
            "arcu",
            "odio",
            "duis",
            "erat",
            "proin",
            "tellus",
            "lectus",
            "etiam",
            "vitae",
            "pulvinar",
            "magna",
            "turpis",
            "velit",
            "curabitur",
            "mauris",
            "suscipit",
            "turpis",
            "congue",
            "dui",
            "suscipit",
            "neque",
            "quam",
            "amet",
            "vestibulum",
            "quam",
            "class",
            "suspendisse",
            "torquent",
            "risus",
            "nisi",
            "laoreet",
            "tellus",
            "lectus",
            "interdum",
            "habitant",
            "molestie",
            "commodo",
            "tristique",
            "scelerisque",
        ],
        vec![
            "malesuada",
            "curabitur",
            "erat",
            "mauris",
            "tempor",
            "vel",
            "leo",
            "condimentum",
            "class",
            "pharetra",
            "platea",
            "justo",
            "rutrum",
            "egestas",
            "magna",
            "interdum",
            "per",
            "dolor",
            "aliquet",
            "vitae",
            "hac",
            "eleifend",
            "torquent",
            "maximus",
            "felis",
            "nullam",
            "ante",
            "elit",
            "etiam",
            "nisl",
            "mattis",
            "consectetur",
            "ipsum",
            "nisl",
            "semper",
            "accumsan",
        ],
        vec![
            "diam",
            "pulvinar",
            "conubia",
            "purus",
            "libero",
            "consequat",
            "turpis",
            "vulputate",
            "hac",
            "donec",
            "porttitor",
            "eleifend",
            "rhoncus",
            "volutpat",
            "leo",
            "sapien",
            "condimentum",
            "conubia",
            "rhoncus",
            "torquent",
            "commodo",
            "non",
        ],
        vec![
            "morbi", "etiam", "porta", "ornare", "posuere", "integer", "etiam", "purus",
            "pulvinar", "nullam", "justo", "amet", "arcu", "dictum", "vitae", "quisque", "etiam",
        ],
        vec![
            "feugiat",
            "massa",
            "maximus",
            "ultrices",
            "ipsum",
            "nec",
            "dui",
            "enim",
            "orci",
            "convallis",
            "est",
            "rhoncus",
            "nisi",
            "congue",
            "felis",
            "nisi",
            "euismod",
        ],
        vec![
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
        ],
        vec![
            "ante",
            "risus",
            "convallis",
            "ad",
            "euismod",
            "velit",
            "eleifend",
            "aenean",
            "fames",
            "curabitur",
            "semper",
            "lorem",
            "aptent",
            "tempus",
        ],
        vec![
            "accumsan",
            "maximus",
            "ultricies",
            "pretium",
            "condimentum",
            "molestie",
            "porta",
            "nec",
            "malesuada",
            "tempor",
        ],
        vec![
            "porttitor",
            "lacinia",
            "porttitor",
            "varius",
            "eleifend",
            "tempus",
            "curabitur",
        ],
        vec![
            "duis",
            "pulvinar",
            "taciti",
            "potenti",
            "vestibulum",
            "aptent",
            "hac",
            "purus",
            "maximus",
            "ut",
            "fringilla",
            "quisque",
            "donec",
            "tortor",
            "suspendisse",
        ],
    ]
}

#[test]
fn test_unigram_frequency_surprisal() -> Result<(), Box<dyn Error>> {
    let mut model = NGramFrequencySurprisal::new(1);
    assert_eq!(
        None,
        model.surprisal(&[[
            "lorem",
            "ipsum",
            "dolor",
            "sit",
            "amet",
            "consectetur",
            "adipiscing",
        ]])
    );
    let texts = get_texts();
    model.fit(&texts);
    assert_eq!(
        Some(vec![
            vec![
                ("lorem".to_string(), 7.05528243550119),
                ("ipsum".to_string(), 6.640244936222346),
                ("dolor".to_string(), 6.640244936222346),
                ("sit".to_string(), 6.640244936222346),
                ("amet".to_string(), 6.318316841334983),
                ("consectetur".to_string(), 6.318316841334983),
                ("adipiscing".to_string(), 6.640244936222346),
            ],
            vec![
                ("feugiat".to_string(), 8.640244936222345),
                ("massa".to_string(), 7.05528243550119),
                ("maximus".to_string(), 6.05528243550119),
                ("ultrices".to_string(), 8.640244936222345),
                ("ipsum".to_string(), 6.640244936222346),
                ("nec".to_string(), 7.640244936222346),
            ],
        ]),
        model.surprisal(&vec![
            vec![
                "lorem",
                "ipsum",
                "dolor",
                "sit",
                "amet",
                "consectetur",
                "adipiscing",
            ],
            vec!["feugiat", "massa", "maximus", "ultrices", "ipsum", "nec",],
        ])
    );
    assert_eq!(
        Some(vec![vec![
            ("lorem".to_string(), 7.05528243550119),
            ("ipsum".to_string(), 6.640244936222346),
            ("dolor".to_string(), 6.640244936222346),
            ("sit".to_string(), 6.640244936222346),
            ("foo".to_string(), 8.643856189774725),
            ("bar".to_string(), 8.643856189774725),
        ]]),
        model.surprisal(&[["lorem", "ipsum", "dolor", "sit", "foo", "bar"]])
    );

    let dir = TempDir::new("ngram_test")?;
    let path = dir.path().join("unigram-frequency-surprisal.json");
    model.save(&path)?;

    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    assert_eq!(
        "{\"n\":1,\"counts\":{\"a\":2,\"ac\":1,\"accumsan\":2,\"ad\":1,\"adipiscing\":4,\"aenean\":3,\"aliqua\":2,\"aliquet\":2,\"amet\":5,\"ante\":5,\"aptent\":3,\"arcu\":4,\"at\":3,\"augue\":1,\"bibendum\":2,\"blandit\":1,\"class\":3,\"commodo\":2,\"condimentum\":4,\"congue\":2,\"consectetur\":5,\"consequat\":4,\"conubia\":2,\"convallis\":3,\"cras\":1,\"cubilia\":2,\"curabitur\":5,\"cursus\":1,\"dapibus\":3,\"diam\":1,\"dictum\":1,\"dictumst\":1,\"dignissim\":3,\"do\":2,\"dolor\":4,\"dolore\":2,\"donec\":4,\"dui\":3,\"duis\":3,\"egestas\":2,\"eget\":2,\"eiusmod\":2,\"eleifend\":4,\"elementum\":2,\"elit\":4,\"enim\":1,\"erat\":2,\"eros\":3,\"est\":2,\"et\":3,\"etiam\":6,\"eu\":2,\"euismod\":2,\"facilisis\":4,\"fames\":1,\"felis\":4,\"fermentum\":4,\"feugiat\":1,\"fringilla\":3,\"fusce\":3,\"gravida\":1,\"habitant\":1,\"habitasse\":1,\"hac\":5,\"hendrerit\":1,\"himenaeos\":1,\"iaculis\":1,\"imperdiet\":1,\"in\":2,\"incididunt\":2,\"integer\":1,\"interdum\":2,\"ipsum\":4,\"justo\":3,\"labore\":2,\"lacinia\":1,\"laoreet\":4,\"lectus\":5,\"leo\":2,\"libero\":2,\"litora\":3,\"lobortis\":2,\"lorem\":3,\"luctus\":1,\"magna\":4,\"malesuada\":4,\"massa\":3,\"mattis\":1,\"mauris\":5,\"maximus\":6,\"metus\":2,\"mi\":2,\"molestie\":2,\"morbi\":1,\"nam\":1,\"nec\":2,\"neque\":2,\"netus\":2,\"nisi\":7,\"nisl\":3,\"non\":3,\"nulla\":2,\"nullam\":6,\"nunc\":2,\"odio\":1,\"orci\":2,\"ornare\":1,\"pellentesque\":1,\"per\":1,\"pharetra\":1,\"platea\":1,\"porta\":3,\"porttitor\":5,\"posuere\":3,\"potenti\":2,\"praesent\":2,\"pretium\":2,\"proin\":3,\"pulvinar\":6,\"purus\":6,\"quam\":5,\"quis\":1,\"quisque\":2,\"rhoncus\":3,\"risus\":2,\"rutrum\":1,\"sagittis\":1,\"sapien\":2,\"scelerisque\":1,\"sed\":3,\"semper\":3,\"senectus\":1,\"sit\":4,\"sodales\":2,\"sollicitudin\":3,\"suscipit\":3,\"suspendisse\":2,\"taciti\":1,\"tellus\":3,\"tempor\":5,\"tempus\":2,\"torquent\":3,\"tortor\":1,\"tristique\":3,\"turpis\":4,\"ultrices\":1,\"ultricies\":4,\"urna\":1,\"ut\":3,\"varius\":1,\"vehicula\":1,\"vel\":2,\"velit\":2,\"venenatis\":1,\"vestibulum\":3,\"vitae\":5,\"vivamus\":1,\"volutpat\":2,\"vulputate\":1},\"suffix_counts\":{}}".to_string(),
        s,
    );

    let model2 = NGramFrequencySurprisal::load(&path)?;
    assert_eq!(model2, model);

    Ok(())
}

#[test]
fn test_bigram_surprisal() -> Result<(), Box<dyn Error>> {
    let mut model = NGramFrequencySurprisal::new(2);
    assert_eq!(
        None,
        model.surprisal(&[[
            "lorem",
            "ipsum",
            "dolor",
            "sit",
            "amet",
            "consectetur",
            "adipiscing",
        ]])
    );
    let texts = get_texts();
    model.fit(&texts);
    assert_eq!(
        Some(vec![
            vec![
                ("lorem".to_string(), 2.9068905956085187),
                ("ipsum".to_string(), 0.5849625007211563),
                ("dolor".to_string(), 1.0),
                ("sit".to_string(), 1.0),
                ("amet".to_string(), 1.0),
                ("consectetur".to_string(), 1.3219280948873622),
                ("adipiscing".to_string(), 1.3219280948873622),
            ],
            vec![
                ("feugiat".to_string(), 3.9068905956085187),
                ("massa".to_string(), 0.0),
                ("maximus".to_string(), 1.5849625007211563),
                ("ultrices".to_string(), 2.584962500721156),
                ("ipsum".to_string(), 0.0),
                ("nec".to_string(), 2.0),
            ],
        ]),
        model.surprisal(&vec![
            vec![
                "lorem",
                "ipsum",
                "dolor",
                "sit",
                "amet",
                "consectetur",
                "adipiscing",
            ],
            vec!["feugiat", "massa", "maximus", "ultrices", "ipsum", "nec",],
        ])
    );
    assert_eq!(
        Some(vec![vec![
            ("lorem".to_string(), 2.9068905956085187),
            ("ipsum".to_string(), 0.5849625007211563),
            ("dolor".to_string(), 1.0),
            ("sit".to_string(), 1.0),
            ("amet".to_string(), 1.0),
            ("foo".to_string(), 2.584962500721156),
            ("bar".to_string(), 8.643856189774725),
        ]]),
        model.surprisal(&[["lorem", "ipsum", "dolor", "sit", "amet", "foo", "bar"]])
    );

    let dir = TempDir::new("ngram_test")?;
    let path = dir.path().join("bigram-frequency-surprisal.json");
    model.save(&path)?;

    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    assert_eq!(
        "{\"n\":2,\"counts\":{\"a\":2,\"ac\":1,\"accumsan\":2,\"ad\":1,\"adipiscing\":4,\"aenean\":3,\"aliqua\":2,\"aliquet\":2,\"amet\":5,\"ante\":5,\"aptent\":3,\"arcu\":4,\"at\":3,\"augue\":1,\"bibendum\":2,\"blandit\":1,\"class\":3,\"commodo\":2,\"condimentum\":4,\"congue\":2,\"consectetur\":5,\"consequat\":4,\"conubia\":2,\"convallis\":3,\"cras\":1,\"cubilia\":2,\"curabitur\":5,\"cursus\":1,\"dapibus\":3,\"diam\":1,\"dictum\":1,\"dictumst\":1,\"dignissim\":3,\"do\":2,\"dolor\":4,\"dolore\":2,\"donec\":4,\"dui\":3,\"duis\":3,\"egestas\":2,\"eget\":2,\"eiusmod\":2,\"eleifend\":4,\"elementum\":2,\"elit\":4,\"enim\":1,\"erat\":2,\"eros\":3,\"est\":2,\"et\":3,\"etiam\":6,\"eu\":2,\"euismod\":2,\"facilisis\":4,\"fames\":1,\"felis\":4,\"fermentum\":4,\"feugiat\":1,\"fringilla\":3,\"fusce\":3,\"gravida\":1,\"habitant\":1,\"habitasse\":1,\"hac\":5,\"hendrerit\":1,\"himenaeos\":1,\"iaculis\":1,\"imperdiet\":1,\"in\":2,\"incididunt\":2,\"integer\":1,\"interdum\":2,\"ipsum\":4,\"justo\":3,\"labore\":2,\"lacinia\":1,\"laoreet\":4,\"lectus\":5,\"leo\":2,\"libero\":2,\"litora\":3,\"lobortis\":2,\"lorem\":3,\"luctus\":1,\"magna\":4,\"malesuada\":4,\"massa\":3,\"mattis\":1,\"mauris\":5,\"maximus\":6,\"metus\":2,\"mi\":2,\"molestie\":2,\"morbi\":1,\"nam\":1,\"nec\":2,\"neque\":2,\"netus\":2,\"nisi\":7,\"nisl\":3,\"non\":3,\"nulla\":2,\"nullam\":6,\"nunc\":2,\"odio\":1,\"orci\":2,\"ornare\":1,\"pellentesque\":1,\"per\":1,\"pharetra\":1,\"platea\":1,\"porta\":3,\"porttitor\":5,\"posuere\":3,\"potenti\":2,\"praesent\":2,\"pretium\":2,\"proin\":3,\"pulvinar\":6,\"purus\":6,\"quam\":5,\"quis\":1,\"quisque\":2,\"rhoncus\":3,\"risus\":2,\"rutrum\":1,\"sagittis\":1,\"sapien\":2,\"scelerisque\":1,\"sed\":3,\"semper\":3,\"senectus\":1,\"sit\":4,\"sodales\":2,\"sollicitudin\":3,\"suscipit\":3,\"suspendisse\":2,\"taciti\":1,\"tellus\":3,\"tempor\":5,\"tempus\":2,\"torquent\":3,\"tortor\":1,\"tristique\":3,\"turpis\":4,\"ultrices\":1,\"ultricies\":4,\"urna\":1,\"ut\":3,\"varius\":1,\"vehicula\":1,\"vel\":2,\"velit\":2,\"venenatis\":1,\"vestibulum\":3,\"vitae\":5,\"vivamus\":1,\"volutpat\":2,\"vulputate\":1},\"suffix_counts\":{\"a\":{\"bibendum\":1,\"dapibus\":1},\"ac\":{\"est\":1},\"accumsan\":{\"_\":1,\"semper\":1},\"ad\":{\"convallis\":1},\"adipiscing\":{\"consectetur\":2,\"nam\":1,\"proin\":1},\"aenean\":{\"at\":1,\"eleifend\":1,\"vestibulum\":1},\"aliqua\":{\"magna\":2},\"aliquet\":{\"dolor\":1,\"felis\":1},\"amet\":{\"felis\":1,\"justo\":1,\"quam\":1,\"sit\":2},\"ante\":{\"_\":1,\"fermentum\":1,\"nisi\":1,\"nullam\":1,\"pulvinar\":1},\"aptent\":{\"consequat\":1,\"lorem\":1,\"vestibulum\":1},\"arcu\":{\"amet\":1,\"massa\":1,\"sollicitudin\":1,\"turpis\":1},\"at\":{\"eu\":1,\"potenti\":1,\"tempor\":1},\"augue\":{\"sodales\":1},\"bibendum\":{\"consectetur\":1,\"ultricies\":1},\"blandit\":{\"laoreet\":1},\"class\":{\"a\":1,\"condimentum\":1,\"quam\":1},\"commodo\":{\"molestie\":1,\"torquent\":1},\"condimentum\":{\"habitasse\":1,\"leo\":1,\"pretium\":1,\"sapien\":1},\"congue\":{\"nisi\":1,\"turpis\":1},\"consectetur\":{\"amet\":2,\"arcu\":1,\"donec\":1,\"mattis\":1},\"consequat\":{\"adipiscing\":1,\"ante\":1,\"class\":1,\"libero\":1},\"conubia\":{\"condimentum\":1,\"pulvinar\":1},\"convallis\":{\"_\":1,\"orci\":1,\"risus\":1},\"cras\":{\"venenatis\":1},\"cubilia\":{\"pulvinar\":1,\"sapien\":1},\"curabitur\":{\"fames\":1,\"malesuada\":1,\"tempus\":1,\"vel\":1,\"velit\":1},\"cursus\":{\"elementum\":1},\"dapibus\":{\"dolor\":1,\"nisi\":1,\"sit\":1},\"diam\":{\"_\":1},\"dictum\":{\"arcu\":1},\"dictumst\":{\"lectus\":1},\"dignissim\":{\"ante\":1,\"fringilla\":1,\"laoreet\":1},\"do\":{\"sed\":2},\"dolor\":{\"dignissim\":1,\"ipsum\":2,\"per\":1},\"dolore\":{\"et\":2},\"donec\":{\"fusce\":1,\"hac\":1,\"quisque\":1,\"ultricies\":1},\"dui\":{\"_\":1,\"congue\":1,\"nec\":1},\"duis\":{\"_\":1,\"justo\":1,\"odio\":1},\"egestas\":{\"porttitor\":1,\"rutrum\":1},\"eget\":{\"nullam\":2},\"eiusmod\":{\"do\":2},\"eleifend\":{\"hac\":1,\"porttitor\":1,\"varius\":1,\"velit\":1},\"elementum\":{\"posuere\":1,\"sodales\":1},\"elit\":{\"adipiscing\":2,\"ante\":1,\"fusce\":1},\"enim\":{\"dui\":1},\"erat\":{\"curabitur\":1,\"duis\":1},\"eros\":{\"elit\":1,\"hac\":1,\"nulla\":1},\"est\":{\"convallis\":1,\"eget\":1},\"et\":{\"labore\":2,\"senectus\":1},\"etiam\":{\"elit\":1,\"integer\":1,\"lectus\":1,\"morbi\":1,\"quisque\":1,\"sollicitudin\":1},\"eu\":{\"imperdiet\":1,\"metus\":1},\"euismod\":{\"ad\":1,\"nisi\":1},\"facilisis\":{\"elementum\":1,\"mauris\":1,\"purus\":1,\"tellus\":1},\"fames\":{\"aenean\":1},\"felis\":{\"congue\":1,\"consequat\":1,\"maximus\":1,\"sollicitudin\":1},\"fermentum\":{\"cursus\":1,\"facilisis\":1,\"iaculis\":1,\"vitae\":1},\"feugiat\":{\"_\":1},\"fringilla\":{\"convallis\":1,\"donec\":1,\"ut\":1},\"fusce\":{\"ac\":1,\"aenean\":1,\"nulla\":1},\"gravida\":{\"at\":1},\"habitant\":{\"interdum\":1},\"habitasse\":{\"curabitur\":1},\"hac\":{\"aptent\":1,\"fusce\":1,\"tristique\":1,\"vitae\":1,\"vulputate\":1},\"hendrerit\":{\"malesuada\":1},\"himenaeos\":{\"quis\":1},\"iaculis\":{\"lobortis\":1},\"imperdiet\":{\"in\":1},\"in\":{\"litora\":1,\"posuere\":1},\"incididunt\":{\"tempor\":2},\"integer\":{\"posuere\":1},\"interdum\":{\"lectus\":1,\"magna\":1},\"ipsum\":{\"consectetur\":1,\"lorem\":2,\"ultrices\":1},\"justo\":{\"eget\":1,\"nullam\":1,\"platea\":1},\"labore\":{\"ut\":2},\"lacinia\":{\"porttitor\":1},\"laoreet\":{\"amet\":1,\"blandit\":1,\"facilisis\":1,\"nisi\":1},\"lectus\":{\"aliqua\":1,\"consectetur\":1,\"hendrerit\":1,\"tellus\":2},\"leo\":{\"vel\":1,\"volutpat\":1},\"libero\":{\"purus\":1,\"sed\":1},\"litora\":{\"massa\":1,\"porttitor\":1,\"tristique\":1},\"lobortis\":{\"fringilla\":1,\"mi\":1},\"lorem\":{\"_\":2,\"semper\":1},\"luctus\":{\"facilisis\":1},\"magna\":{\"dolore\":2,\"egestas\":1,\"pulvinar\":1},\"malesuada\":{\"_\":1,\"facilisis\":1,\"gravida\":1,\"nec\":1},\"massa\":{\"dignissim\":1,\"feugiat\":1,\"non\":1},\"mattis\":{\"nisl\":1},\"mauris\":{\"cras\":1,\"curabitur\":1,\"erat\":1,\"mi\":1,\"volutpat\":1},\"maximus\":{\"accumsan\":1,\"massa\":1,\"nullam\":1,\"pretium\":1,\"purus\":1,\"torquent\":1},\"metus\":{\"condimentum\":1,\"nisi\":1},\"mi\":{\"et\":1,\"vitae\":1},\"molestie\":{\"condimentum\":1,\"habitant\":1},\"morbi\":{\"_\":1},\"nam\":{\"himenaeos\":1},\"nec\":{\"ipsum\":1,\"porta\":1},\"neque\":{\"dapibus\":1,\"suscipit\":1},\"netus\":{\"augue\":1,\"pellentesque\":1},\"nisi\":{\"adipiscing\":1,\"felis\":1,\"purus\":2,\"rhoncus\":1,\"risus\":1,\"vivamus\":1},\"nisl\":{\"etiam\":1,\"ipsum\":1,\"litora\":1},\"non\":{\"commodo\":1,\"fermentum\":1,\"sagittis\":1},\"nulla\":{\"consequat\":1,\"duis\":1},\"nullam\":{\"aliquet\":1,\"felis\":1,\"netus\":1,\"non\":1,\"pulvinar\":1,\"quam\":1},\"nunc\":{\"dapibus\":1,\"proin\":1},\"odio\":{\"arcu\":1},\"orci\":{\"enim\":1,\"in\":1},\"ornare\":{\"porta\":1},\"pellentesque\":{\"suscipit\":1},\"per\":{\"interdum\":1},\"pharetra\":{\"class\":1},\"platea\":{\"pharetra\":1},\"porta\":{\"etiam\":1,\"molestie\":1,\"netus\":1},\"porttitor\":{\"_\":1,\"cubilia\":1,\"donec\":1,\"lacinia\":1,\"laoreet\":1},\"posuere\":{\"arcu\":1,\"luctus\":1,\"ornare\":1},\"potenti\":{\"eros\":1,\"taciti\":1},\"praesent\":{\"dictumst\":1,\"nullam\":1},\"pretium\":{\"eu\":1,\"ultricies\":1},\"proin\":{\"dui\":1,\"egestas\":1,\"erat\":1},\"pulvinar\":{\"diam\":1,\"duis\":1,\"eros\":1,\"metus\":1,\"purus\":1,\"vitae\":1},\"purus\":{\"conubia\":1,\"etiam\":1,\"hac\":1,\"maximus\":1,\"nunc\":1,\"quam\":1},\"quam\":{\"_\":1,\"aptent\":1,\"neque\":1,\"nisl\":1,\"vestibulum\":1},\"quis\":{\"orci\":1},\"quisque\":{\"fringilla\":1,\"vitae\":1},\"rhoncus\":{\"conubia\":1,\"eleifend\":1,\"est\":1},\"risus\":{\"ante\":1,\"torquent\":1},\"rutrum\":{\"justo\":1},\"sagittis\":{\"_\":1},\"sapien\":{\"leo\":1,\"neque\":1},\"scelerisque\":{\"tristique\":1},\"sed\":{\"elit\":2,\"ultricies\":1},\"semper\":{\"curabitur\":1,\"mauris\":1,\"nisl\":1},\"senectus\":{\"fermentum\":1},\"sit\":{\"_\":1,\"aenean\":1,\"dolor\":2},\"sodales\":{\"lectus\":1,\"praesent\":1},\"sollicitudin\":{\"ante\":1,\"semper\":1,\"sit\":1},\"suscipit\":{\"dui\":1,\"mauris\":2},\"suspendisse\":{\"class\":1,\"tortor\":1},\"taciti\":{\"pulvinar\":1},\"tellus\":{\"eros\":1,\"laoreet\":1,\"proin\":1},\"tempor\":{\"bibendum\":1,\"eiusmod\":2,\"malesuada\":1,\"mauris\":1},\"tempus\":{\"aptent\":1,\"eleifend\":1},\"torquent\":{\"eleifend\":1,\"rhoncus\":1,\"suspendisse\":1},\"tortor\":{\"donec\":1},\"tristique\":{\"commodo\":1,\"maximus\":1,\"vehicula\":1},\"turpis\":{\"consequat\":1,\"fermentum\":1,\"magna\":1,\"suscipit\":1},\"ultrices\":{\"maximus\":1},\"ultricies\":{\"cubilia\":1,\"litora\":1,\"maximus\":1,\"quam\":1},\"urna\":{\"etiam\":1},\"ut\":{\"incididunt\":2,\"maximus\":1},\"varius\":{\"porttitor\":1},\"vehicula\":{\"lobortis\":1},\"vel\":{\"praesent\":1,\"tempor\":1},\"velit\":{\"euismod\":1,\"turpis\":1},\"venenatis\":{\"nunc\":1},\"vestibulum\":{\"amet\":1,\"hac\":1,\"potenti\":1},\"vitae\":{\"aliquet\":1,\"dictum\":1,\"etiam\":1,\"lectus\":1,\"porta\":1},\"vivamus\":{\"a\":1},\"volutpat\":{\"rhoncus\":1,\"urna\":1},\"vulputate\":{\"turpis\":1}}}".to_string(),
        s,
    );

    let model2 = NGramFrequencySurprisal::load(&path)?;
    assert_eq!(model2, model);

    Ok(())
}

#[test]
fn test_trigram_frequency_surprisal() -> Result<(), Box<dyn Error>> {
    let mut model = NGramFrequencySurprisal::new(3);
    assert_eq!(
        None,
        model.surprisal(&[[
            "lorem",
            "ipsum",
            "dolor",
            "sit",
            "amet",
            "consectetur",
            "adipiscing",
        ]])
    );
    let texts = get_texts();
    model.fit(&texts);
    assert_eq!(
        Some(vec![
            vec![
                ("lorem".to_string(), 2.9068905956085187),
                ("ipsum".to_string(), 0.0),
                ("dolor".to_string(), 0.0),
                ("sit".to_string(), 0.0),
                ("amet".to_string(), 0.0),
                ("consectetur".to_string(), 0.0),
                ("adipiscing".to_string(), 0.0),
            ],
            vec![
                ("feugiat".to_string(), 3.9068905956085187),
                ("massa".to_string(), 0.0),
                ("maximus".to_string(), 0.0),
                ("ultrices".to_string(), 0.0),
                ("ipsum".to_string(), 0.0),
                ("nec".to_string(), 0.0),
            ],
        ]),
        model.surprisal(&vec![
            vec![
                "lorem",
                "ipsum",
                "dolor",
                "sit",
                "amet",
                "consectetur",
                "adipiscing",
            ],
            vec!["feugiat", "massa", "maximus", "ultrices", "ipsum", "nec",],
        ])
    );
    assert_eq!(
        Some(vec![vec![
            ("lorem".to_string(), 2.9068905956085187),
            ("ipsum".to_string(), 0.0),
            ("dolor".to_string(), 0.0),
            ("sit".to_string(), 0.0),
            ("amet".to_string(), 0.0),
            ("foo".to_string(), 1.5849625007211563),
            ("bar".to_string(), 8.643856189774725),
        ]]),
        model.surprisal(&[["lorem", "ipsum", "dolor", "sit", "amet", "foo", "bar"]])
    );

    let dir = TempDir::new("ngram_test")?;
    let path = dir.path().join("trigram-frequency-surprisal.json");
    model.save(&path)?;

    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    assert_eq!(
        "{\"n\":3,\"counts\":{\"a\":2,\"ac\":1,\"accumsan\":2,\"ad\":1,\"adipiscing\":4,\"aenean\":3,\"aliqua\":2,\"aliquet\":2,\"amet\":5,\"ante\":5,\"aptent\":3,\"arcu\":4,\"at\":3,\"augue\":1,\"bibendum\":2,\"blandit\":1,\"class\":3,\"commodo\":2,\"condimentum\":4,\"congue\":2,\"consectetur\":5,\"consequat\":4,\"conubia\":2,\"convallis\":3,\"cras\":1,\"cubilia\":2,\"curabitur\":5,\"cursus\":1,\"dapibus\":3,\"diam\":1,\"dictum\":1,\"dictumst\":1,\"dignissim\":3,\"do\":2,\"dolor\":4,\"dolore\":2,\"donec\":4,\"dui\":3,\"duis\":3,\"egestas\":2,\"eget\":2,\"eiusmod\":2,\"eleifend\":4,\"elementum\":2,\"elit\":4,\"enim\":1,\"erat\":2,\"eros\":3,\"est\":2,\"et\":3,\"etiam\":6,\"eu\":2,\"euismod\":2,\"facilisis\":4,\"fames\":1,\"felis\":4,\"fermentum\":4,\"feugiat\":1,\"fringilla\":3,\"fusce\":3,\"gravida\":1,\"habitant\":1,\"habitasse\":1,\"hac\":5,\"hendrerit\":1,\"himenaeos\":1,\"iaculis\":1,\"imperdiet\":1,\"in\":2,\"incididunt\":2,\"integer\":1,\"interdum\":2,\"ipsum\":4,\"justo\":3,\"labore\":2,\"lacinia\":1,\"laoreet\":4,\"lectus\":5,\"leo\":2,\"libero\":2,\"litora\":3,\"lobortis\":2,\"lorem\":3,\"luctus\":1,\"magna\":4,\"malesuada\":4,\"massa\":3,\"mattis\":1,\"mauris\":5,\"maximus\":6,\"metus\":2,\"mi\":2,\"molestie\":2,\"morbi\":1,\"nam\":1,\"nec\":2,\"neque\":2,\"netus\":2,\"nisi\":7,\"nisl\":3,\"non\":3,\"nulla\":2,\"nullam\":6,\"nunc\":2,\"odio\":1,\"orci\":2,\"ornare\":1,\"pellentesque\":1,\"per\":1,\"pharetra\":1,\"platea\":1,\"porta\":3,\"porttitor\":5,\"posuere\":3,\"potenti\":2,\"praesent\":2,\"pretium\":2,\"proin\":3,\"pulvinar\":6,\"purus\":6,\"quam\":5,\"quis\":1,\"quisque\":2,\"rhoncus\":3,\"risus\":2,\"rutrum\":1,\"sagittis\":1,\"sapien\":2,\"scelerisque\":1,\"sed\":3,\"semper\":3,\"senectus\":1,\"sit\":4,\"sodales\":2,\"sollicitudin\":3,\"suscipit\":3,\"suspendisse\":2,\"taciti\":1,\"tellus\":3,\"tempor\":5,\"tempus\":2,\"torquent\":3,\"tortor\":1,\"tristique\":3,\"turpis\":4,\"ultrices\":1,\"ultricies\":4,\"urna\":1,\"ut\":3,\"varius\":1,\"vehicula\":1,\"vel\":2,\"velit\":2,\"venenatis\":1,\"vestibulum\":3,\"vitae\":5,\"vivamus\":1,\"volutpat\":2,\"vulputate\":1},\"suffix_counts\":{\"a\":{\"consectetur_bibendum\":1,\"dolor_dapibus\":1},\"ac\":{\"eget_est\":1},\"accumsan\":{\"___\":1,\"nisl_semper\":1},\"ad\":{\"risus_convallis\":1},\"adipiscing\":{\"amet_consectetur\":2,\"dui_proin\":1,\"himenaeos_nam\":1},\"aenean\":{\"eu_at\":1,\"hac_vestibulum\":1,\"velit_eleifend\":1},\"aliqua\":{\"dolore_magna\":2},\"aliquet\":{\"per_dolor\":1,\"sollicitudin_felis\":1},\"amet\":{\"consequat_felis\":1,\"dolor_sit\":2,\"neque_quam\":1,\"nullam_justo\":1},\"ante\":{\"___\":1,\"felis_nullam\":1,\"iaculis_fermentum\":1,\"metus_pulvinar\":1,\"purus_nisi\":1},\"aptent\":{\"ante_consequat\":1,\"potenti_vestibulum\":1,\"semper_lorem\":1},\"arcu\":{\"dignissim_massa\":1,\"fermentum_turpis\":1,\"justo_amet\":1,\"sit_sollicitudin\":1},\"at\":{\"bibendum_tempor\":1,\"eros_potenti\":1,\"imperdiet_eu\":1},\"augue\":{\"praesent_sodales\":1},\"bibendum\":{\"arcu_consectetur\":1,\"cubilia_ultricies\":1},\"blandit\":{\"amet_laoreet\":1},\"class\":{\"bibendum_a\":1,\"leo_condimentum\":1,\"vestibulum_quam\":1},\"commodo\":{\"habitant_molestie\":1,\"rhoncus_torquent\":1},\"condimentum\":{\"curabitur_habitasse\":1,\"leo_sapien\":1,\"ultricies_pretium\":1,\"vel_leo\":1},\"congue\":{\"rhoncus_nisi\":1,\"suscipit_turpis\":1},\"consectetur\":{\"nisl_mattis\":1,\"sit_amet\":2,\"sollicitudin_arcu\":1,\"ultricies_donec\":1},\"consequat\":{\"a_class\":1,\"fermentum_ante\":1,\"proin_adipiscing\":1,\"purus_libero\":1},\"conubia\":{\"diam_pulvinar\":1,\"sapien_condimentum\":1},\"convallis\":{\"___\":1,\"ante_risus\":1,\"enim_orci\":1},\"cras\":{\"nunc_venenatis\":1},\"cubilia\":{\"eros_pulvinar\":1,\"neque_sapien\":1},\"curabitur\":{\"__malesuada\":1,\"aenean_fames\":1,\"eleifend_tempus\":1,\"praesent_vel\":1,\"turpis_velit\":1},\"cursus\":{\"posuere_elementum\":1},\"dapibus\":{\"__sit\":1,\"dignissim_dolor\":1,\"purus_nisi\":1},\"diam\":{\"___\":1},\"dictum\":{\"amet_arcu\":1},\"dictumst\":{\"hendrerit_lectus\":1},\"dignissim\":{\"convallis_fringilla\":1,\"facilisis_laoreet\":1,\"pulvinar_ante\":1},\"do\":{\"elit_sed\":2},\"dolor\":{\"ante_dignissim\":1,\"interdum_per\":1,\"lorem_ipsum\":2},\"dolore\":{\"labore_et\":2},\"donec\":{\"fringilla_quisque\":1,\"litora_ultricies\":1,\"nulla_fusce\":1,\"vulputate_hac\":1},\"dui\":{\"___\":1,\"ipsum_nec\":1,\"turpis_congue\":1},\"duis\":{\"___\":1,\"arcu_odio\":1,\"eget_justo\":1},\"egestas\":{\"cubilia_porttitor\":1,\"justo_rutrum\":1},\"eget\":{\"netus_nullam\":1,\"quam_nullam\":1},\"eiusmod\":{\"sed_do\":2},\"eleifend\":{\"donec_porttitor\":1,\"euismod_velit\":1,\"porttitor_varius\":1,\"vitae_hac\":1},\"elementum\":{\"lectus_sodales\":1,\"luctus_posuere\":1},\"elit\":{\"ac_fusce\":1,\"consectetur_adipiscing\":2,\"nullam_ante\":1},\"enim\":{\"nec_dui\":1},\"erat\":{\"malesuada_curabitur\":1,\"odio_duis\":1},\"eros\":{\"duis_nulla\":1,\"fusce_elit\":1,\"fusce_hac\":1},\"est\":{\"nullam_eget\":1,\"orci_convallis\":1},\"et\":{\"fermentum_senectus\":1,\"ut_labore\":2},\"etiam\":{\"__morbi\":1,\"ante_elit\":1,\"ante_sollicitudin\":1,\"posuere_integer\":1,\"tellus_lectus\":1,\"vitae_quisque\":1},\"eu\":{\"condimentum_metus\":1,\"in_imperdiet\":1},\"euismod\":{\"convallis_ad\":1,\"felis_nisi\":1},\"facilisis\":{\"eros_tellus\":1,\"mi_mauris\":1,\"quam_purus\":1,\"sodales_elementum\":1},\"fames\":{\"eleifend_aenean\":1},\"felis\":{\"adipiscing_consequat\":1,\"nisi_congue\":1,\"semper_sollicitudin\":1,\"torquent_maximus\":1},\"fermentum\":{\"elementum_cursus\":1,\"elementum_facilisis\":1,\"lobortis_iaculis\":1,\"porta_vitae\":1},\"feugiat\":{\"___\":1},\"fringilla\":{\"__convallis\":1,\"fusce_donec\":1,\"maximus_ut\":1},\"fusce\":{\"consequat_nulla\":1,\"est_ac\":1,\"vestibulum_aenean\":1},\"gravida\":{\"tempor_at\":1},\"habitant\":{\"lectus_interdum\":1},\"habitasse\":{\"vel_curabitur\":1},\"hac\":{\"aenean_fusce\":1,\"aliquet_vitae\":1,\"turpis_vulputate\":1,\"vehicula_tristique\":1,\"vestibulum_aptent\":1},\"hendrerit\":{\"facilisis_malesuada\":1},\"himenaeos\":{\"orci_quis\":1},\"iaculis\":{\"mi_lobortis\":1},\"imperdiet\":{\"posuere_in\":1},\"in\":{\"arcu_posuere\":1,\"porttitor_litora\":1},\"incididunt\":{\"eiusmod_tempor\":2},\"integer\":{\"ornare_posuere\":1},\"interdum\":{\"egestas_magna\":1,\"tellus_lectus\":1},\"ipsum\":{\"__lorem\":2,\"mattis_consectetur\":1,\"maximus_ultrices\":1},\"justo\":{\"nullam_eget\":1,\"pharetra_platea\":1,\"pulvinar_nullam\":1},\"labore\":{\"incididunt_ut\":2},\"lacinia\":{\"__porttitor\":1},\"laoreet\":{\"felis_amet\":1,\"laoreet_blandit\":1,\"purus_facilisis\":1,\"risus_nisi\":1},\"lectus\":{\"donec_consectetur\":1,\"laoreet_tellus\":1,\"magna_aliqua\":1,\"malesuada_hendrerit\":1,\"proin_tellus\":1},\"leo\":{\"rhoncus_volutpat\":1,\"tempor_vel\":1},\"libero\":{\"conubia_purus\":1,\"ultricies_sed\":1},\"litora\":{\"laoreet_porttitor\":1,\"maximus_tristique\":1,\"non_massa\":1},\"lobortis\":{\"donec_fringilla\":1,\"et_mi\":1},\"lorem\":{\"___\":2,\"curabitur_semper\":1},\"luctus\":{\"mauris_facilisis\":1},\"magna\":{\"et_dolore\":2,\"rutrum_egestas\":1,\"vitae_pulvinar\":1},\"malesuada\":{\"___\":1,\"at_gravida\":1,\"porta_nec\":1,\"tellus_facilisis\":1},\"massa\":{\"__feugiat\":1,\"fermentum_non\":1,\"fringilla_dignissim\":1},\"mattis\":{\"etiam_nisl\":1},\"mauris\":{\"curabitur_erat\":1,\"urna_volutpat\":1,\"velit_curabitur\":1,\"venenatis_cras\":1,\"vitae_mi\":1},\"maximus\":{\"__accumsan\":1,\"eleifend_torquent\":1,\"eu_pretium\":1,\"feugiat_massa\":1,\"hac_purus\":1,\"non_nullam\":1},\"metus\":{\"adipiscing_nisi\":1,\"habitasse_condimentum\":1},\"mi\":{\"lectus_vitae\":1,\"senectus_et\":1},\"molestie\":{\"interdum_habitant\":1,\"pretium_condimentum\":1},\"morbi\":{\"___\":1},\"nam\":{\"quis_himenaeos\":1},\"nec\":{\"molestie_porta\":1,\"ultrices_ipsum\":1},\"neque\":{\"dui_suscipit\":1,\"sit_dapibus\":1},\"netus\":{\"sodales_augue\":1,\"suscipit_pellentesque\":1},\"nisi\":{\"a_vivamus\":1,\"congue_felis\":1,\"est_rhoncus\":1,\"maximus_purus\":1,\"nam_adipiscing\":1,\"nunc_purus\":1,\"torquent_risus\":1},\"nisl\":{\"consectetur_ipsum\":1,\"elit_etiam\":1,\"tristique_litora\":1},\"non\":{\"__sagittis\":1,\"cursus_fermentum\":1,\"torquent_commodo\":1},\"nulla\":{\"class_consequat\":1,\"justo_duis\":1},\"nullam\":{\"__quam\":1,\"augue_netus\":1,\"felis_aliquet\":1,\"maximus_felis\":1,\"purus_pulvinar\":1,\"sagittis_non\":1},\"nunc\":{\"egestas_proin\":1,\"nisi_dapibus\":1},\"odio\":{\"turpis_arcu\":1},\"orci\":{\"dui_enim\":1,\"litora_in\":1},\"ornare\":{\"etiam_porta\":1},\"pellentesque\":{\"mauris_suscipit\":1},\"per\":{\"magna_interdum\":1},\"pharetra\":{\"condimentum_class\":1},\"platea\":{\"class_pharetra\":1},\"porta\":{\"condimentum_molestie\":1,\"morbi_etiam\":1,\"pellentesque_netus\":1},\"porttitor\":{\"___\":1,\"blandit_laoreet\":1,\"hac_donec\":1,\"porttitor_lacinia\":1,\"sapien_cubilia\":1},\"posuere\":{\"facilisis_luctus\":1,\"massa_arcu\":1,\"porta_ornare\":1},\"potenti\":{\"nulla_eros\":1,\"pulvinar_taciti\":1},\"praesent\":{\"aliquet_nullam\":1,\"lectus_dictumst\":1},\"pretium\":{\"maximus_ultricies\":1,\"metus_eu\":1},\"proin\":{\"__dui\":1,\"duis_erat\":1,\"porttitor_egestas\":1},\"pulvinar\":{\"__diam\":1,\"__duis\":1,\"etiam_purus\":1,\"etiam_vitae\":1,\"hac_eros\":1,\"nisi_metus\":1},\"purus\":{\"aptent_hac\":1,\"integer_etiam\":1,\"nisl_quam\":1,\"nullam_maximus\":1,\"proin_nunc\":1,\"pulvinar_conubia\":1},\"quam\":{\"___\":1,\"amet_vestibulum\":1,\"consequat_aptent\":1,\"litora_nisl\":1,\"suscipit_neque\":1},\"quis\":{\"in_orci\":1},\"quisque\":{\"dictum_vitae\":1,\"ut_fringilla\":1},\"rhoncus\":{\"condimentum_conubia\":1,\"convallis_est\":1,\"porttitor_eleifend\":1},\"risus\":{\"__ante\":1,\"suspendisse_torquent\":1},\"rutrum\":{\"platea_justo\":1},\"sagittis\":{\"___\":1},\"sapien\":{\"dapibus_neque\":1,\"volutpat_leo\":1},\"scelerisque\":{\"commodo_tristique\":1},\"sed\":{\"adipiscing_elit\":2,\"quam_ultricies\":1},\"semper\":{\"fames_curabitur\":1,\"ipsum_nisl\":1,\"volutpat_mauris\":1},\"senectus\":{\"facilisis_fermentum\":1},\"sit\":{\"___\":1,\"at_aenean\":1,\"ipsum_dolor\":2},\"sodales\":{\"consectetur_lectus\":1,\"dictumst_praesent\":1},\"sollicitudin\":{\"aenean_sit\":1,\"mauris_semper\":1,\"nisi_ante\":1},\"suscipit\":{\"congue_dui\":1,\"cras_mauris\":1,\"curabitur_mauris\":1},\"suspendisse\":{\"donec_tortor\":1,\"quam_class\":1},\"taciti\":{\"duis_pulvinar\":1},\"tellus\":{\"elit_eros\":1,\"erat_proin\":1,\"nisi_laoreet\":1},\"tempor\":{\"do_eiusmod\":2,\"erat_mauris\":1,\"nec_malesuada\":1,\"ultricies_bibendum\":1},\"tempus\":{\"lorem_aptent\":1,\"varius_eleifend\":1},\"torquent\":{\"class_suspendisse\":1,\"conubia_rhoncus\":1,\"hac_eleifend\":1},\"tortor\":{\"quisque_donec\":1},\"tristique\":{\"lobortis_vehicula\":1,\"molestie_commodo\":1,\"pretium_maximus\":1},\"turpis\":{\"libero_consequat\":1,\"mauris_suscipit\":1,\"pulvinar_magna\":1,\"vitae_fermentum\":1},\"ultrices\":{\"massa_maximus\":1},\"ultricies\":{\"accumsan_maximus\":1,\"aptent_quam\":1,\"massa_litora\":1,\"pulvinar_cubilia\":1},\"urna\":{\"sollicitudin_etiam\":1},\"ut\":{\"purus_maximus\":1,\"tempor_incididunt\":2},\"varius\":{\"lacinia_porttitor\":1},\"vehicula\":{\"fringilla_lobortis\":1},\"vel\":{\"mauris_tempor\":1,\"nullam_praesent\":1},\"velit\":{\"ad_euismod\":1,\"magna_turpis\":1},\"venenatis\":{\"dapibus_nunc\":1},\"vestibulum\":{\"quam_amet\":1,\"taciti_potenti\":1,\"tristique_hac\":1},\"vitae\":{\"aliqua_lectus\":1,\"arcu_dictum\":1,\"dolor_aliquet\":1,\"lectus_etiam\":1,\"netus_porta\":1},\"vivamus\":{\"dapibus_a\":1},\"volutpat\":{\"eleifend_rhoncus\":1,\"etiam_urna\":1},\"vulputate\":{\"consequat_turpis\":1}}}".to_string(),
        s,
    );

    let model2 = NGramFrequencySurprisal::load(&path)?;
    assert_eq!(model2, model);

    Ok(())
}

#[test]
fn test_ngramsurprisal_load() -> Result<(), Box<dyn Error>> {
    let tmp_dir = TempDir::new("ngram_test")?;
    let path = tmp_dir.path().join("ngram_surprisal.json");

    let mut f = File::create(&path)?;
    f.write_all(b"[]")?;

    let r = NGramFrequencySurprisal::load(&path);
    assert!(r.is_err());
    assert!(matches!(r, Err(LoadError::ParseError)));

    let mut f = File::create(&path)?;
    f.write_all(b"{}")?;

    let r = NGramFrequencySurprisal::load(&path);
    assert!(r.is_err());
    assert!(matches!(r, Err(LoadError::MissingKeyError(_))));

    let mut f = File::create(&path)?;
    f.write_all(b"{\"n\":2}")?;

    let r = NGramFrequencySurprisal::load(&path);
    assert!(r.is_err());
    assert!(matches!(r, Err(LoadError::MissingKeyError(_))));

    let mut f = File::create(&path)?;
    f.write_all(b"{\"n\":2,\"counts\":{}}")?;

    let r = NGramFrequencySurprisal::load(&path);
    assert!(r.is_err());
    assert!(matches!(r, Err(LoadError::MissingKeyError(_))));

    let mut f = File::create(&path)?;
    f.write_all(b"{\"n\":2,\"counts\":{},\"suffix_counts\":{}}")?;

    let r = NGramFrequencySurprisal::load(&path);
    assert!(r.is_ok());
    assert_eq!(NGramFrequencySurprisal::new(2), r.unwrap());

    let mut f = File::create(&path)?;
    f.write_all(
        b"{\"n\":1,\"counts\":{\"foo\":1,\"bar\":2},\"suffix_counts\":{\"bar\":{\"foo\":2}}}",
    )?;

    let r = NGramFrequencySurprisal::load(&path);
    assert!(r.is_err());
    assert!(matches!(r, Err(LoadError::ParseError)));

    Ok(())
}
