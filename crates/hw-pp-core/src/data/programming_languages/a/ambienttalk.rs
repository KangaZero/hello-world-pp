//! Hello-world snippet for `ProgrammingLanguageName::Ambienttalk`.
//!
//! Imported from leachim6/hello-world (PR #1662 `a/AmbientTalk.at`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1662>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ambienttalk => "AmbientTalk",
    versions: [Ambienttalk => "AmbientTalk"],
    extensions: [At => "at"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ambienttalk,
        extensions: &[FileExtension::At],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ambienttalk,
            syntax: concat!(r#"system.println(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"system.println(""#, output_for_hello_world!(), r#"");"#),
            )],
        }],
    };
