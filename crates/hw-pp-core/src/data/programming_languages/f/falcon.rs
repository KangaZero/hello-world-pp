//! Hello-world snippet for `ProgrammingLanguageName::Falcon`.
//!
//! Imported from leachim6/hello-world (`f/Falcon.fal` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Falcon.fal>
//! - <https://www.wikidata.org/wiki/Q580448> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{ObjectOriented, Procedural};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Falcon => "Falcon",
    versions: [Falcon => "Falcon"],
    extensions: [Fal => "fal"],
}

const FALCON: &str = concat!(
    r#"#!/usr/bin/env falcon

> ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Falcon,
        extensions: &[FileExtension::Fal],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Procedural, ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Falcon,
            syntax: FALCON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FALCON,
                    [
                        (TokenKind::Macro, r"#!/usr/bin/env falcon"),
                        (
                            TokenKind::Plain,
                            r"

> ",
                        ),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
