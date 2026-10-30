//! Hello-world snippet for `ProgrammingLanguageName::Luau`.
//!
//! Imported from leachim6/hello-world (`l/Luau.luau` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Luau.luau>
//! - <https://en.wikipedia.org/wiki/Luau_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Imperative, Procedural};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Luau => "Luau",
    versions: [Luau => "Luau"],
    extensions: [Luau => "luau"],
}

const LUAU: &str = concat!(r"print('", output_for_hello_world!(), r"')");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Luau,
        extensions: &[FileExtension::Luau],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, Procedural, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Luau,
            syntax: LUAU,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LUAU,
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
