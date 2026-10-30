//! Hello-world snippet for `ProgrammingLanguageName::Gambas`.
//!
//! Imported from the Hello World Collection (`files/Gambas.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Gambas.txt>
//! - <http://helloworldcollection.de/#Gambas>
//! - <https://en.wikipedia.org/wiki/Gambas> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Gambas => "Gambas",
    versions: [Gambas => "Gambas"],
    extensions: [],
}

const GAMBAS: &str = concat!(
    r"'************************************
' ",
    output_for_hello_world!(),
    r#" in Gambas
'************************************
PUBLIC SUB Main()

   PRINT ""#,
    output_for_hello_world!(),
    r#""

END"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gambas,
        extensions: &[],
        categories: &[],
        families: &[Basic],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gambas,
            syntax: GAMBAS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GAMBAS,
                    [
                        (TokenKind::Comment, r"'************************************"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            concat!(r"' ", output_for_hello_world!(), r" in Gambas"),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"'************************************"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"PUBLIC"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"SUB"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"Main"),
                        (TokenKind::Punctuation, r"()"),
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
