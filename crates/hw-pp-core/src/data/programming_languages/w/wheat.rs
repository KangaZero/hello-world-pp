//! Hello-world snippet for `ProgrammingLanguageName::Wheat`.
//!
//! Imported from leachim6/hello-world (`w/Wheat.wheat` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Wheat => "Wheat",
    versions: [Wheat => "Wheat"],
    extensions: [Wheat => "wheat"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wheat,
        extensions: &[FileExtension::Wheat],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wheat,
            syntax: r#"input h
if h "H":
 output " world!"
 output N
 terminate
output "Hello,""#,
            tokenized_syntax: &[(
                TokenKind::Plain,
                r#"input h
if h "H":
 output " world!"
 output N
 terminate
output "Hello,""#,
            )],
        }],
    };
