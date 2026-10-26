//! Hello-world snippet for `ProgrammingLanguageName::Gofer`.
//!
//! Imported from the Hello World Collection (`files/Gofer` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Gofer>
//! - <http://helloworldcollection.de/#Gofer>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Haskell,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Gofer => "Gofer",
    versions: [Gofer => "Gofer"],
    extensions: [],
}

const GOFER: &str = concat!(
    r"-- ",
    output_for_hello_world!(),
    r#" in Gofer
-- Simple version

helloWorld:: String
helloWorld = ""#,
    output_for_hello_world!(),
    r#"\n"


-- "#,
    output_for_hello_world!(),
    r#" in Gofer
-- Dialog version

helloWorld :: Dialogue
helloWorld resps = [AppendChan stdout ""#,
    output_for_hello_world!(),
    r#""]"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gofer,
        extensions: &[],
        categories: &[],
        families: &[Haskell],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gofer,
            syntax: GOFER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GOFER,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r"-- ", output_for_hello_world!(), r" in Gofer"),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"-- Simple version"),
                        (
                            TokenKind::Plain,
                            r"

helloWorld",
                        ),
                        (TokenKind::Punctuation, r"::"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"String"),
                        (
                            TokenKind::Plain,
                            r"
helloWorld = ",
                        ),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"


",
                        ),
                        (
                            TokenKind::Comment,
                            concat!(r"-- ", output_for_hello_world!(), r" in Gofer"),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"-- Dialog version"),
                        (
                            TokenKind::Plain,
                            r"

helloWorld ",
                        ),
                        (TokenKind::Punctuation, r"::"),
                        (
                            TokenKind::Plain,
                            r" Dialogue
helloWorld resps = ",
                        ),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Plain, r"AppendChan stdout "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r"]"),
                    ],
                )
            },
        }],
    };
