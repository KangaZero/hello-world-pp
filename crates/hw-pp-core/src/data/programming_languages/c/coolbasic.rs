//! Hello-world snippet for `ProgrammingLanguageName::Coolbasic`.
//!
//! Imported from the Hello World Collection (`files/CoolBasic.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/CoolBasic.txt>
//! - <http://helloworldcollection.de/#CoolBasic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Coolbasic => "CoolBasic",
    versions: [Coolbasic => "CoolBasic"],
    extensions: [],
}

const COOLBASIC: &str = concat!(
    r"' ",
    output_for_hello_world!(),
    r#" in CoolBasic

print ""#,
    output_for_hello_world!(),
    r#""
wait key"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Coolbasic,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Coolbasic,
            syntax: COOLBASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COOLBASIC,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r"' ", output_for_hello_world!(), r" in CoolBasic"),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"print"),
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
                        (TokenKind::Keyword, r"wait"),
                        (TokenKind::Plain, r" key"),
                    ],
                )
            },
        }],
    };
