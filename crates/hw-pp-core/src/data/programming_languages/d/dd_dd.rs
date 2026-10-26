//! Hello-world snippet for `ProgrammingLanguageName::DdDd`.
//!
//! Imported from leachim6/hello-world (`d/DD DD.dd` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/DD%20DD.dd>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: DdDd => "DD DD",
    versions: [DdDd => "DD DD"],
    extensions: [Dd => "dd"],
}

const DD_DD: &str = r"<!Hello\ World\0A;";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::DdDd,
        extensions: &[FileExtension::Dd],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::DdDd,
            syntax: DD_DD,
            tokenized_syntax: &const {
                checked_token_lengths(DD_DD, [(TokenKind::Plain, r"<!Hello\ World\0A;")])
            },
        }],
    };
