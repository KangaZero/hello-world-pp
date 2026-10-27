//! Hello-world snippet for `ProgrammingLanguageName::SmalltalkGnu`.
//!
//! Imported from leachim6/hello-world (`s/SmallTalk GNU.st` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SmallTalk%20GNU.st>
//! - <https://en.wikipedia.org/wiki/Smalltalk> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: SmalltalkGnu => "SmallTalk GNU",
    versions: [SmalltalkGnu => "SmallTalk GNU"],
    extensions: [],
}

const SMALLTALK_GNU: &str = concat!(r"'", output_for_hello_world!(), r"' printNl !");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SmalltalkGnu,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SmalltalkGnu,
            syntax: SMALLTALK_GNU,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SMALLTALK_GNU,
                    [
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"printNl"),
                        (TokenKind::Plain, r" !"),
                    ],
                )
            },
        }],
    };
