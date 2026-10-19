//! Hello-world snippet for `ProgrammingLanguageName::Gta3script`.
//!
//! Imported from leachim6/hello-world (`g/GTA3Script.sc` at `a152253`),
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
    language: Gta3script => "GTA3Script",
    versions: [Gta3script => "GTA3Script"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gta3script,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gta3script,
            syntax: concat!(
                r#"SCRIPT_START
{
NOP

WAIT 0
PRINT_STRING_NOW ""#,
                output_for_hello_world!(),
                r#""
}
SCRIPT_END"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"SCRIPT_START
{
NOP

WAIT 0
PRINT_STRING_NOW ""#,
                    output_for_hello_world!(),
                    r#""
}
SCRIPT_END"#
                ),
            )],
        }],
    };
