//! Hello-world snippet for `ProgrammingLanguageName::Vjass`.
//!
//! Imported from leachim6/hello-world (`v/VJass.j` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/VJass.j>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Vjass => "VJass",
    versions: [Vjass => "VJass"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vjass,
        extensions: &[],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vjass,
            syntax: concat!(
                r#"struct HelloWorld extends array
    private static method onInit takes nothing returns nothing
        call DisplayTimedTextToPlayer(GetLocalPlayer(), 0, 0, 0, ""#,
                output_for_hello_world!(),
                r#"")
    endmethod
endstruct"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"struct HelloWorld extends array
    private static method onInit takes nothing returns nothing
        call DisplayTimedTextToPlayer(GetLocalPlayer(), 0, 0, 0, ""#,
                    output_for_hello_world!(),
                    r#"")
    endmethod
endstruct"#
                ),
            )],
        }],
    };
