//! Hello-world snippet for `ProgrammingLanguageName::Blitzplus`.
//!
//! Imported from the Hello World Collection (`files/BlitzPlus.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/BlitzPlus.txt>
//! - <http://helloworldcollection.de/#BlitzPlus>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Blitzplus => "BlitzPlus",
    versions: [Blitzplus => "BlitzPlus"],
    extensions: [],
}

const BLITZPLUS: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r#" in Blitz Plus (graphical mode)

Graphics 800,600,0,1
Text 790, 600, ""#,
    output_for_hello_world!(),
    r#""
WaitKey"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Blitzplus,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Blitzplus,
            syntax: BLITZPLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BLITZPLUS,
                    [
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            concat!(r" ", output_for_hello_world!(), r" in Blitz "),
                        ),
                        (TokenKind::Function, r"Plus"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"graphical mode"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"Graphics"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"800"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"600"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"1"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"Text"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"790"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"600"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"WaitKey"),
                    ],
                )
            },
        }],
    };
