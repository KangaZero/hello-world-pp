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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: LibreofficeBasic => "LibreOffice Basic",
    versions: [LibreofficeBasic => "LibreOffice Basic"],
    extensions: [],
}

const LIBREOFFICE_BASIC: &str = concat!(
    "\u{feff}REM  *****  BASIC  *****\n\nSub Main\n msgbox \"",
    output_for_hello_world!(),
    r#""
End Sub"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LibreofficeBasic,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LibreofficeBasic,
            syntax: LIBREOFFICE_BASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LIBREOFFICE_BASIC,
                    [
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
                )
            },
        }],
    };
