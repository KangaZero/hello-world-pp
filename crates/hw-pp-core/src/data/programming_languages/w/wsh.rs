//! Hello-world snippet for `ProgrammingLanguageName::Wsh`.
//!
//! Imported from leachim6/hello-world (`w/WSH.vbs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/WSH.vbs>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Wsh => "WSH",
    versions: [Wsh => "WSH"],
    extensions: [],
}

const WSH: &str = concat!(
    r#"WScript.Echo ""#,
    output_for_hello_world!(),
    r#""
WScript.Quit"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wsh,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wsh,
            syntax: WSH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WSH,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"WScript.Echo ""#,
                            output_for_hello_world!(),
                            r#""
WScript.Quit"#
                        ),
                    )],
                )
            },
        }],
    };
