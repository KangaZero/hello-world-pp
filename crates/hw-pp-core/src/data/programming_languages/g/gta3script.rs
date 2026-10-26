//! Hello-world snippet for `ProgrammingLanguageName::Gta3script`.
//!
//! Imported from leachim6/hello-world (`g/GTA3Script.sc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/GTA3Script.sc>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Gta3script => "GTA3Script",
    versions: [Gta3script => "GTA3Script"],
    extensions: [],
}

const GTA3SCRIPT: &str = concat!(
    r#"SCRIPT_START
{
NOP

WAIT 0
PRINT_STRING_NOW ""#,
    output_for_hello_world!(),
    r#""
}
SCRIPT_END"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gta3script,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gta3script,
            syntax: GTA3SCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GTA3SCRIPT,
                    [(
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
                )
            },
        }],
    };
