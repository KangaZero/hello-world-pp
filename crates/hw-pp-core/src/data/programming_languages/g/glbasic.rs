//! Hello-world snippet for `ProgrammingLanguageName::Glbasic`.
//!
//! Imported from the Hello World Collection (`files/GLBasic.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/GLBasic.htm>
//! - <http://helloworldcollection.de/#GLBasic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Glbasic => "GLBasic",
    versions: [Glbasic => "GLBasic"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Glbasic,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Glbasic,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r#" in GLBasic

PRINT ""#,
                output_for_hello_world!(),
                r#"",1,1
SHOWSCREEN
MOUSEWAIT"#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Plain,
                    concat!(
                        r"// ",
                        output_for_hello_world!(),
                        r" in GLBasic

"
                    ),
                ),
                (TokenKind::Keyword, r"PRINT"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r","),
                (TokenKind::Number, r"1"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Number, r"1"),
                (
                    TokenKind::Plain,
                    r"
SHOWSCREEN
MOUSEWAIT",
                ),
            ],
        }],
    };
