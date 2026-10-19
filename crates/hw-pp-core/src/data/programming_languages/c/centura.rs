//! Hello-world snippet for `ProgrammingLanguageName::Centura`.
//!
//! Imported from leachim6/hello-world (`c/Centura.centura` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Centura.centura>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Centura => "Centura",
    versions: [Centura => "Centura"],
    extensions: [Centura => "centura"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Centura,
        extensions: &[FileExtension::Centura],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Centura,
            syntax: concat!(
                r"! ",
                output_for_hello_world!(),
                r#" in Centura

Function: HelloWorld
Description:
Returns
Parameters
Static Variables
Local variables
Actions
 Call SalMessageBox( ""#,
                output_for_hello_world!(),
                r#"",'Message',MB_Ok)"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"! ",
                    output_for_hello_world!(),
                    r#" in Centura

Function: HelloWorld
Description:
Returns
Parameters
Static Variables
Local variables
Actions
 Call SalMessageBox( ""#,
                    output_for_hello_world!(),
                    r#"",'Message',MB_Ok)"#
                ),
            )],
        }],
    };
