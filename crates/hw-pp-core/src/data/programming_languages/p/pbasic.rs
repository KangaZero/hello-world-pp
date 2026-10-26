//! Hello-world snippet for `ProgrammingLanguageName::Pbasic`.
//!
//! Imported from the Hello World Collection (`files/PBASIC.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/PBASIC.txt>
//! - <http://helloworldcollection.de/#PBASIC>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Pbasic => "PBASIC",
    versions: [Pbasic => "PBASIC"],
    extensions: [],
}

const PBASIC: &str = concat!(
    r"' ",
    output_for_hello_world!(),
    r#" in PBASIC (for the Boe-Bot Robot)

' {$STAMP BS2}
' {$PBASIC 2.5}

DEBUG ""#,
    output_for_hello_world!(),
    r#""

END"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pbasic,
        extensions: &[],
        categories: &[],
        families: &[Basic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pbasic,
            syntax: PBASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PBASIC,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r"' ",
                                output_for_hello_world!(),
                                r" in PBASIC (for the Boe-Bot Robot)"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Comment, r"' {$STAMP BS2}"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"' {$PBASIC 2.5}"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"DEBUG"),
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
                        (TokenKind::Keyword, r"END"),
                    ],
                )
            },
        }],
    };
