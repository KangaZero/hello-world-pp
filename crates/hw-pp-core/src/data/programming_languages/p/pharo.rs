//! Hello-world snippet for `ProgrammingLanguageName::Pharo`.
//!
//! Imported from leachim6/hello-world (`p/Pharo.st` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pharo.st>
//! - <https://www.wikidata.org/wiki/Q369096> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{ObjectOriented, Reflective};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Smalltalk, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Pharo => "Pharo",
    versions: [Pharo => "Pharo"],
    extensions: [],
}

const PHARO: &str = concat!(r"'", output_for_hello_world!(), r"' crLog");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pharo,
        extensions: &[],
        categories: &[Scripting],
        families: &[Smalltalk],
        paradigms: &[ObjectOriented, Reflective],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pharo,
            syntax: PHARO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PHARO,
                    [
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Plain, r" crLog"),
                    ],
                )
            },
        }],
    };
