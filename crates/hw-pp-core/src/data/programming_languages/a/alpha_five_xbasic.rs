//! Hello-world snippet for `ProgrammingLanguageName::AlphaFiveXbasic`.
//!
//! Imported from the Hello World Collection (`files/Alpha-Five-Xbasic.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Alpha-Five-Xbasic.txt>
//! - <http://helloworldcollection.de/#Alpha-Five-Xbasic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AlphaFiveXbasic => "Alpha-Five-Xbasic",
    versions: [AlphaFiveXbasic => "Alpha-Five-Xbasic"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AlphaFiveXbasic,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AlphaFiveXbasic,
            syntax: concat!(
                r"' ",
                output_for_hello_world!(),
                r#" in Alpha Five Xbasic

ui_msg_box("The '"#,
                output_for_hello_world!(),
                r#"' Collection", ""#,
                output_for_hello_world!(),
                r#"", UI_ATTENTION_SYMBOL)"#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    concat!(r"' ", output_for_hello_world!(), r" in Alpha Five Xbasic"),
                ),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Function, r"ui_msg_box"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#""The '"#, output_for_hello_world!(), r#"' Collection""#),
                ),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" UI_ATTENTION_SYMBOL"),
                (TokenKind::Punctuation, r")"),
            ],
        }],
    };
