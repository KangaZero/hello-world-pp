//! Hello-world snippet for `ProgrammingLanguageName::Jason`.
//!
//! Imported from leachim6/hello-world (`j/Jason.mas2j` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/Jason.mas2j>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Jason => "Jason",
    versions: [Jason => "Jason"],
    extensions: [Mas2j => "mas2j"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jason,
        extensions: &[FileExtension::Mas2j],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jason,
            syntax: concat!(
                r#"!greeting.
+!greeting : true <- .print(""#,
                output_for_hello_world!(),
                r#"")."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"!greeting.
+!greeting : true <- .print(""#,
                    output_for_hello_world!(),
                    r#"")."#
                ),
            )],
        }],
    };
