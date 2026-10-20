//! Hello-world snippet for `ProgrammingLanguageName::Obix`.
//!
//! Imported from leachim6/hello-world (`o/Obix.osc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Obix.osc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Obix => "Obix",
    versions: [Obix => "Obix"],
    extensions: [Osc => "osc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Obix,
        extensions: &[FileExtension::Osc],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Obix,
            syntax: concat!(
                r#"system.console.write_line ( ""#,
                output_for_hello_world!(),
                r#"" )"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"system.console.write_line ( ""#,
                    output_for_hello_world!(),
                    r#"" )"#
                ),
            )],
        }],
    };
