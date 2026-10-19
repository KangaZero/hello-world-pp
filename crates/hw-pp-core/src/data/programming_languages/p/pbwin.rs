//! Hello-world snippet for `ProgrammingLanguageName::Pbwin`.
//!
//! Imported from leachim6/hello-world (`p/PBWin.bas` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PBWin.bas>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Pbwin => "PBWin",
    versions: [Pbwin => "PBWin"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pbwin,
        extensions: &[],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pbwin,
            syntax: concat!(
                r#"#COMPILE EXE
#DIM ALL

FUNCTION PBMAIN () AS LONG

    MSGBOX ""#,
                output_for_hello_world!(),
                r#""

END FUNCTION"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#COMPILE EXE
#DIM ALL

FUNCTION PBMAIN () AS LONG

    MSGBOX ""#,
                    output_for_hello_world!(),
                    r#""

END FUNCTION"#
                ),
            )],
        }],
    };
