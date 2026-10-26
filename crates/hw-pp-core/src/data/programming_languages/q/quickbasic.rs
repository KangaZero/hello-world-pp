//! Hello-world snippet for `ProgrammingLanguageName::Quickbasic`.
//!
//! Imported from the Hello World Collection (`files/QuickBASIC.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/QuickBASIC.txt>
//! - <http://helloworldcollection.de/#QuickBASIC>
//! - <https://en.wikipedia.org/wiki/QuickBASIC> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Procedural;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Quickbasic => "QuickBASIC",
    versions: [Quickbasic => "QuickBASIC"],
    extensions: [],
}

const QUICKBASIC: &str = concat!(
    r"REM ",
    output_for_hello_world!(),
    r#" in QuickBASIC
PRINT ""#,
    output_for_hello_world!(),
    r#""
END"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Quickbasic,
        extensions: &[],
        categories: &[],
        families: &[Basic],
        paradigms: &[Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Quickbasic,
            syntax: QUICKBASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    QUICKBASIC,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r"REM ", output_for_hello_world!(), r" in QuickBASIC"),
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
