//! Hello-world snippets for `ProgrammingLanguageName::V`.
//!
//! Imported from leachim6/hello-world (`v/V`, `v/V.v` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/V>
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/V.v>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::C, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: V => "V",
    versions: [V1 => "V (1)", VV => "V (.v)"],
    extensions: [],
}

const V1: &str = concat!(r"i", output_for_hello_world!());
const VV: &str = concat!(r#"println(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::V,
        extensions: &[],
        categories: &[Compiled],
        families: &[C],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::V1,
                syntax: V1,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        V1,
                        [(TokenKind::Plain, concat!(r"i", output_for_hello_world!()))],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::VV,
                syntax: VV,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        VV,
                        [
                            (TokenKind::Function, r"println"),
                            (TokenKind::Punctuation, r"("),
                            (
                                TokenKind::String,
                                concat!(r#"""#, output_for_hello_world!(), r#"""#),
                            ),
                            (TokenKind::Punctuation, r")"),
                        ],
                    )
                },
            },
        ],
    };
