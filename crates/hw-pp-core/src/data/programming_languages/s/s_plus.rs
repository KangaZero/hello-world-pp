//! Hello-world snippet for `ProgrammingLanguageName::SPlus`.
//!
//! Imported from the Hello World Collection (`files/S-Plus.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/S-Plus.txt>
//! - <http://helloworldcollection.de/#S-Plus>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: SPlus => "S-Plus",
    versions: [SPlus => "S-Plus"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SPlus,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SPlus,
            syntax: concat!(
                r"# ",
                output_for_hello_world!(),
                r#" for S-Plus
cat(""#,
                output_for_hello_world!(),
                r#"\n")"#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    concat!(r"# ", output_for_hello_world!(), r" for S-Plus"),
                ),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Function, r"cat"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Punctuation, r")"),
            ],
        }],
    };
