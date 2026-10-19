//! Hello-world snippet for `ProgrammingLanguageName::Angelscript`.
//!
//! Imported from leachim6/hello-world (`a/AngelScript` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/AngelScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Angelscript => "AngelScript",
    versions: [Angelscript => "AngelScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Angelscript,
        extensions: &[],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Angelscript,
            syntax: concat!(
                r#"void main() { print(""#,
                output_for_hello_world!(),
                r#"\n"); }"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"void main() { print(""#,
                    output_for_hello_world!(),
                    r#"\n"); }"#
                ),
            )],
        }],
    };
