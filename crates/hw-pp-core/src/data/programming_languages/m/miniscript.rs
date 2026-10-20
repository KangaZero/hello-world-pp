//! Hello-world snippet for `ProgrammingLanguageName::Miniscript`.
//!
//! Imported from the Hello World Collection (`files/MiniScript.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MiniScript.htm>
//! - <http://helloworldcollection.de/#MiniScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Miniscript => "MiniScript",
    versions: [Miniscript => "MiniScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Miniscript,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Miniscript,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r#" in MiniScript

print ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    concat!(r"// ", output_for_hello_world!(), r" in MiniScript"),
                ),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Function, r"print"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
            ],
        }],
    };
