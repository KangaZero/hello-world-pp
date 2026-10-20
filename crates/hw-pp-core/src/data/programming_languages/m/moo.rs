//! Hello-world snippet for `ProgrammingLanguageName::Moo`.
//!
//! Imported from leachim6/hello-world (`m/Moo.moo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Moo.moo>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Moo => "Moo",
    versions: [Moo => "Moo"],
    extensions: [Moo => "moo"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Moo,
        extensions: &[FileExtension::Moo],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Moo,
            syntax: concat!(
                r#"@program hello:run
player:tell(""#,
                output_for_hello_world!(),
                r#"");
."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"@program hello:run
player:tell(""#,
                    output_for_hello_world!(),
                    r#"");
."#
                ),
            )],
        }],
    };
