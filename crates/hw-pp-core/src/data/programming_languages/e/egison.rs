//! Hello-world snippet for `ProgrammingLanguageName::Egison`.
//!
//! Imported from leachim6/hello-world (`e/Egison.egi` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

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
