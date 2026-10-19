//! Hello-world snippet for `ProgrammingLanguageName::LibreofficeBasic`.
//!
//! Imported from leachim6/hello-world (`l/LibreOffice Basic.bas` at `a152253`),
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
    language: LibreofficeBasic => "LibreOffice Basic",
    versions: [LibreofficeBasic => "LibreOffice Basic"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LibreofficeBasic,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LibreofficeBasic,
            syntax: concat!(
                "\u{feff}REM  *****  BASIC  *****\n\nSub Main\n msgbox \"",
                output_for_hello_world!(),
                r#""
End Sub"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    "\u{feff}REM  *****  BASIC  *****\n\nSub Main\n msgbox \"",
                    output_for_hello_world!(),
                    r#""
End Sub"#
                ),
            )],
        }],
    };
