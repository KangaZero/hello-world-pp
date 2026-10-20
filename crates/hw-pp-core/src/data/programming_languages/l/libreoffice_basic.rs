//! Hello-world snippet for `ProgrammingLanguageName::LibreofficeBasic`.
//!
//! Imported from leachim6/hello-world (`l/LibreOffice Basic.bas` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/LibreOffice%20Basic.bas>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: LibreofficeBasic => "LibreOffice Basic",
    versions: [LibreofficeBasic => "LibreOffice Basic"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LibreofficeBasic,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LibreofficeBasic,
            syntax: concat!(
                "\u{feff}REM  *****  BASIC  *****\n\nSub Main\n msgbox \"",
                output_for_hello_world!(),
                r#""
End Sub"#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, "\u{feff}"),
                (TokenKind::Keyword, r"REM"),
                (
                    TokenKind::Plain,
                    r"  *****  BASIC  *****

",
                ),
                (TokenKind::Keyword, r"Sub"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"Main"),
                (
                    TokenKind::Plain,
                    r"
 ",
                ),
                (TokenKind::Keyword, r"msgbox"),
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
                (TokenKind::Keyword, r"End"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"Sub"),
            ],
        }],
    };
