//! Hello-world snippet for `ProgrammingLanguageName::Sparql`.
//!
//! Imported from leachim6/hello-world (`s/SparQL.sparql` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SparQL.sparql>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Query,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Sparql => "SparQL",
    versions: [Sparql => "SparQL"],
    extensions: [Sparql => "sparql"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sparql,
        extensions: &[FileExtension::Sparql],
        categories: &[Query],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sparql,
            syntax: concat!(
                r#"SELECT ?h WHERE { 
  VALUES ?h { ""#,
                output_for_hello_world!(),
                r#"" } 
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"SELECT ?h WHERE { 
  VALUES ?h { ""#,
                    output_for_hello_world!(),
                    r#"" } 
}"#
                ),
            )],
        }],
    };
