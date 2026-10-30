//! Hello-world snippet for `ProgrammingLanguageName::Alice`.
//!
//! Imported from leachim6/hello-world (`a/Alice.alice` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Alice.alice>
//! - <https://esolangs.org/wiki/Alice> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{StackBased, TwoDimensional};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

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
        paradigms: &[StackBased, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Alice,
            syntax: ALICE,
            tokenized_syntax: &const {
                checked_token_lengths(ALICE, [(TokenKind::Plain, r#""dlroW olleH"d&O`@"#)])
            },
        }],
    };
