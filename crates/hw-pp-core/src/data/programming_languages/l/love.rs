//! Hello-world snippet for `ProgrammingLanguageName::Love`.
//!
//! Imported from leachim6/hello-world (`l/Löve.lua` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Love => "Löve",
    versions: [Love => "Löve"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Love,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Love,
            syntax: concat!(
                r#"function love.draw()
    love.graphics.print(""#,
                output_for_hello_world!(),
                r#"", 400, 300)
end"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"function"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"love.draw"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"
    love.graphics.print(",
                ),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Plain, r", "),
                (TokenKind::Number, r"400"),
                (TokenKind::Plain, r", "),
                (TokenKind::Number, r"300"),
                (
                    TokenKind::Plain,
                    r")
",
                ),
                (TokenKind::Keyword, r"end"),
            ],
        }],
    };
