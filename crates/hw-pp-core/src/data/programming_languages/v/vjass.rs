//! Hello-world snippet for `ProgrammingLanguageName::Vjass`.
//!
//! Imported from leachim6/hello-world (`v/VJass.j` at `a152253`),
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
    language: Vjass => "VJass",
    versions: [Vjass => "VJass"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vjass,
        extensions: &[],
        categories: &[],
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
