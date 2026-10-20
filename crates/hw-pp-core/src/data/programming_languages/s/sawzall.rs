//! Hello-world snippet for `ProgrammingLanguageName::Sawzall`.
//!
//! Imported from the Hello World Collection (`files/Sawzall.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Sawzall.htm>
//! - <http://helloworldcollection.de/#Sawzall>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Sawzall => "Sawzall",
    versions: [Sawzall => "Sawzall"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sawzall,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sawzall,
            syntax: concat!(
                r"# ",
                output_for_hello_world!(),
                r#" in Sawzall

emit stdout <- ""#,
                output_for_hello_world!(),
                r#"";"#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    concat!(r"# ", output_for_hello_world!(), r" in Sawzall"),
                ),
                (
                    TokenKind::Plain,
                    r"

emit stdout <- ",
                ),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r";"),
            ],
        }],
    };
