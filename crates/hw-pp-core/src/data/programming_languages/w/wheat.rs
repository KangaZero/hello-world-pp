//! Hello-world snippet for `ProgrammingLanguageName::Wheat`.
//!
//! Imported from leachim6/hello-world (`w/Wheat.wheat` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/Wheat.wheat>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Esolang],
        families: &[],
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
