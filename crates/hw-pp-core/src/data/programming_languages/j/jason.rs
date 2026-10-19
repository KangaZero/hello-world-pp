//! Hello-world snippet for `ProgrammingLanguageName::Jason`.
//!
//! Imported from leachim6/hello-world (`j/Jason.mas2j` at `a152253`),
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
    language: Jason => "Jason",
    versions: [Jason => "Jason"],
    extensions: [Mas2j => "mas2j"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jason,
        extensions: &[FileExtension::Mas2j],
        categories: &[],
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
