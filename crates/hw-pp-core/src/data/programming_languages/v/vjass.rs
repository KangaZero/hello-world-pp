//! Hello-world snippet for `ProgrammingLanguageName::Vjass`.
//!
//! Imported from leachim6/hello-world (`v/VJass.j` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/VJass.j>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Vjass => "VJass",
    versions: [Vjass => "VJass"],
    extensions: [],
}

const VJASS: &str = concat!(
    r#"struct HelloWorld extends array
    private static method onInit takes nothing returns nothing
        call DisplayTimedTextToPlayer(GetLocalPlayer(), 0, 0, 0, ""#,
    output_for_hello_world!(),
    r#"")
    endmethod
endstruct"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vjass,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vjass,
            syntax: VJASS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VJASS,
                    [
                        (TokenKind::Keyword, r"struct"),
                        (TokenKind::Plain, r" HelloWorld "),
                        (TokenKind::Keyword, r"extends"),
                        (
                            TokenKind::Plain,
                            r" array
    ",
                        ),
                        (TokenKind::Keyword, r"private"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"static"),
                        (
                            TokenKind::Plain,
                            r" method onInit takes nothing returns nothing
        call ",
                        ),
                        (TokenKind::Function, r"DisplayTimedTextToPlayer"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"GetLocalPlayer"),
                        (TokenKind::Punctuation, r"(),"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
    endmethod
endstruct",
                        ),
                    ],
                )
            },
        }],
    };
