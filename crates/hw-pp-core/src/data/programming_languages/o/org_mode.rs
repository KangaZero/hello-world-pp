//! Hello-world snippet for `ProgrammingLanguageName::OrgMode`.
//!
//! Imported from leachim6/hello-world (`o/Org-mode.org` at `a152253`),
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
    language: OrgMode => "Org-mode",
    versions: [OrgMode => "Org-mode"],
    extensions: [Org => "org"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OrgMode,
        extensions: &[FileExtension::Org],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OrgMode,
            syntax: output_for_hello_world!(),
            tokenized_syntax: &[(TokenKind::Plain, output_for_hello_world!())],
        }],
    };
