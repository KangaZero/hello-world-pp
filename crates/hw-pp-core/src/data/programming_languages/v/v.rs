//! Hello-world snippet for `ProgrammingLanguageName::V`.
//!
//! Imported from leachim6/hello-world (`v/V`, `v/V.v` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/V>
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/V.v>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: V => "V",
    versions: [V1 => "V (1)", VV => "V (.v)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::V,
        extensions: &[],
        categories: &[Compiled],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::V1,
                syntax: concat!(r"i", output_for_hello_world!()),
                tokenized_syntax: &[(TokenKind::Plain, concat!(r"i", output_for_hello_world!()))],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::VV,
                syntax: concat!(r#"println(""#, output_for_hello_world!(), r#"")"#),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(r#"println(""#, output_for_hello_world!(), r#"")"#),
                )],
            },
        ],
    };
