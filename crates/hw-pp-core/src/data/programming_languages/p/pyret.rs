//! Hello-world snippet for `ProgrammingLanguageName::Pyret`.
//!
//! Imported from leachim6/hello-world (`p/Pyret.arr` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pyret.arr>
//! - <https://pyret.org/> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, ObjectOriented};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Pyret => "Pyret",
    versions: [Pyret => "Pyret"],
    extensions: [Arr => "arr"],
}

const PYRET: &str = concat!(r"print('", output_for_hello_world!(), r"')");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pyret,
        extensions: &[FileExtension::Arr],
        categories: &[Educational],
        families: &[],
        paradigms: &[ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pyret,
            syntax: PYRET,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PYRET,
                    [
                        (TokenKind::Function, r"print"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
