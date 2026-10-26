//! Hello-world snippet for `ProgrammingLanguageName::OrgMode`.
//!
//! Imported from leachim6/hello-world (`o/Org-mode.org` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Org-mode.org>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: OrgMode => "Org-mode",
    versions: [OrgMode => "Org-mode"],
    extensions: [Org => "org"],
}

const ORG_MODE: &str = output_for_hello_world!();

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OrgMode,
        extensions: &[FileExtension::Org],
        categories: &[Markup],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OrgMode,
            syntax: ORG_MODE,
            tokenized_syntax: &const {
                checked_token_lengths(ORG_MODE, [(TokenKind::Plain, output_for_hello_world!())])
            },
        }],
    };
