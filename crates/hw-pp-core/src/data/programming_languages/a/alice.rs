//! Hello-world snippet for `ProgrammingLanguageName::Alice`.
//!
//! Imported from leachim6/hello-world (`a/Alice.alice` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Alice.alice>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Alice => "Alice",
    versions: [Alice => "Alice"],
    extensions: [Alice => "alice"],
}

const ALICE: &str = r#""dlroW olleH"d&O`@"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Alice,
        extensions: &[FileExtension::Alice],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Alice,
            syntax: ALICE,
            tokenized_syntax: &const {
                checked_token_lengths(ALICE, [(TokenKind::Plain, r#""dlroW olleH"d&O`@"#)])
            },
        }],
    };
