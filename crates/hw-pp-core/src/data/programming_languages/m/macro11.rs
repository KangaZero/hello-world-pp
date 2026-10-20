//! Hello-world snippet for `ProgrammingLanguageName::Macro11`.
//!
//! Imported from the Hello World Collection (`files/MACRO-11.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MACRO-11.txt>
//! - <http://helloworldcollection.de/#MACRO-11>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Macro11 => "MACRO-11",
    versions: [Macro11 => "MACRO-11"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Macro11,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Macro11,
            syntax: concat!(
                r#";       ""#,
                output_for_hello_world!(),
                r#"" in MACRO-11 for RT-11

        .MCALL  .EXIT,.PRINT
START:  .PRINT  #$1
        .EXIT
$1:     .ASCIZ  /"#,
                output_for_hello_world!(),
                r"/
        .END    START"
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    concat!(
                        r#";       ""#,
                        output_for_hello_world!(),
                        r#"" in MACRO-11 for RT-11"#
                    ),
                ),
                (
                    TokenKind::Plain,
                    r"

        ",
                ),
                (TokenKind::Macro, r".MCALL"),
                (TokenKind::Plain, r"  "),
                (TokenKind::Macro, r".EXIT"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Macro, r".PRINT"),
                (
                    TokenKind::Plain,
                    r"
START",
                ),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r"  "),
                (TokenKind::Macro, r".PRINT"),
                (TokenKind::Plain, r"  #$"),
                (TokenKind::Number, r"1"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Macro, r".EXIT"),
                (
                    TokenKind::Plain,
                    r"
$",
                ),
                (TokenKind::Number, r"1"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r"     "),
                (TokenKind::Macro, r".ASCIZ"),
                (
                    TokenKind::Plain,
                    concat!(
                        r"  /",
                        output_for_hello_world!(),
                        r"/
        "
                    ),
                ),
                (TokenKind::Macro, r".END"),
                (TokenKind::Plain, r"    START"),
            ],
        }],
    };
