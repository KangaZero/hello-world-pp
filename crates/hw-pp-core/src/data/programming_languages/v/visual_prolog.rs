//! Hello-world snippet for `ProgrammingLanguageName::VisualProlog`.
//!
//! Imported from the Hello World Collection (`files/Visual&nbsp;Prolog.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Visual%26nbsp%3BProlog.htm>
//! - <http://helloworldcollection.de/#Visual%C2%A0Prolog>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Prolog,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: VisualProlog => "Visual Prolog",
    versions: [VisualProlog => "Visual Prolog"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VisualProlog,
        extensions: &[],
        categories: &[],
        families: &[Prolog],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::VisualProlog,
            syntax: concat!(
                r"/* ",
                output_for_hello_world!(),
                r#" in Visual Prolog */

goal
    console::init(),
    stdio::write(""#,
                output_for_hello_world!(),
                r#"")."#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    concat!(r"/* ", output_for_hello_world!(), r" in Visual Prolog */"),
                ),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"goal"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"console"),
                (TokenKind::Punctuation, r"::"),
                (TokenKind::Keyword, r"init"),
                (TokenKind::Punctuation, r"(),"),
                (
                    TokenKind::Plain,
                    r"
    stdio",
                ),
                (TokenKind::Punctuation, r"::"),
                (TokenKind::Keyword, r"write"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")."),
            ],
        }],
    };
