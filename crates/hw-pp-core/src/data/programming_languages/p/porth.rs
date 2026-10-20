//! Hello-world snippet for `ProgrammingLanguageName::Porth`.
//!
//! Imported from leachim6/hello-world (`p/Porth.porth` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Porth.porth>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Porth => "Porth",
    versions: [Porth => "Porth"],
    extensions: [Porth => "porth"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Porth,
        extensions: &[FileExtension::Porth],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Porth,
            syntax: concat!(
                r#"include "std.porth"

proc main in
  ""#,
                output_for_hello_world!(),
                r#"\n" puts
end"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"include "std.porth"

proc main in
  ""#,
                    output_for_hello_world!(),
                    r#"\n" puts
end"#
                ),
            )],
        }],
    };
