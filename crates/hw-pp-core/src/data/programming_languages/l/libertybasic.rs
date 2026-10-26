//! Hello-world snippet for `ProgrammingLanguageName::Libertybasic`.
//!
//! Imported from the Hello World Collection (`files/LibertyBASIC.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/LibertyBASIC.txt>
//! - <http://helloworldcollection.de/#LibertyBASIC>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Libertybasic => "LibertyBASIC",
    versions: [Libertybasic => "LibertyBASIC"],
    extensions: [],
}

const LIBERTYBASIC: &str = concat!(
    r"'",
    output_for_hello_world!(),
    r#" in Liberty BASIC
PRINT ""#,
    output_for_hello_world!(),
    r#""
END"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Libertybasic,
        extensions: &[],
        categories: &[],
        families: &[Basic],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Libertybasic,
            syntax: LIBERTYBASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LIBERTYBASIC,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r"'", output_for_hello_world!(), r" in Liberty BASIC"),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"PRINT"),
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
