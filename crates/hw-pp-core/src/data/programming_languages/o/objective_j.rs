//! Hello-world snippet for `ProgrammingLanguageName::ObjectiveJ`.
//!
//! Imported from leachim6/hello-world (`o/Objective J.j` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Objective%20J.j>
//! - <https://www.wikidata.org/wiki/Q1443431> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented, Reflective,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: ObjectiveJ => "Objective J",
    versions: [ObjectiveJ => "Objective J"],
    extensions: [],
}

const OBJECTIVE_J: &str = concat!(r#"document.write(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ObjectiveJ,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, ObjectOriented, Functional, Reflective],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ObjectiveJ,
            syntax: OBJECTIVE_J,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OBJECTIVE_J,
                    [
                        (TokenKind::Plain, r"document"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"write"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                    ],
                )
            },
        }],
    };
