//! Hello-world snippet for `ProgrammingLanguageName::Flix`.
//!
//! Imported from leachim6/hello-world (`f/Flix.flix` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Flix.flix>
//! - <https://www.wikidata.org/wiki/Q104901822> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Imperative, Logic};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Flix => "Flix",
    versions: [Flix => "Flix"],
    extensions: [Flix => "flix"],
}

const FLIX: &str = concat!(r#"print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Flix,
        extensions: &[FileExtension::Flix],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Imperative, Functional, Logic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Flix,
            syntax: FLIX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FLIX,
                    [
                        (TokenKind::Function, r"print"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
