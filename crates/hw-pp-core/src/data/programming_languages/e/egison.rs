//! Hello-world snippet for `ProgrammingLanguageName::Egison`.
//!
//! Imported from leachim6/hello-world (`e/Egison.egi` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Egison.egi>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Egison => "Egison",
    versions: [Egison => "Egison"],
    extensions: [Egi => "egi"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Egison,
        extensions: &[FileExtension::Egi],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Egison,
            syntax: concat!(
                r#"def main args := write ""#,
                output_for_hello_world!(),
                r#"\n""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"def main args := write ""#,
                    output_for_hello_world!(),
                    r#"\n""#
                ),
            )],
        }],
    };
