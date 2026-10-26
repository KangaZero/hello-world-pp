//! Hello-world snippet for `ProgrammingLanguageName::Emojicoder`.
//!
//! Imported from leachim6/hello-world (`e/EmojiCoder` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/EmojiCoder>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Emojicoder => "EmojiCoder",
    versions: [Emojicoder => "EmojiCoder"],
    extensions: [],
}

const EMOJICODER: &str = r"👋🔡🌚🌝🌚🌚🌝🌚🌚🌚
👋🔡🌚🌝🌝🌚🌚🌝🌚🌝
👋🔡🌚🌝🌝🌚🌝🌝🌚🌚
👋🔡🌚🌝🌝🌚🌝🌝🌚🌚
👋🔡🌚🌝🌝🌚🌝🌝🌝🌝
👋🔡🌚🌚🌝🌚🌚🌚🌚🌚
👋🔡🌚🌝🌚🌝🌚🌝🌝🌝
👋🔡🌚🌝🌝🌚🌝🌝🌝🌝
👋🔡🌚🌝🌝🌝🌚🌚🌝🌚
👋🔡🌚🌝🌝🌚🌝🌝🌚🌚
👋🔡🌚🌝🌝🌚🌚🌝🌚🌚";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Emojicoder,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Emojicoder,
            syntax: EMOJICODER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EMOJICODER,
                    [(
                        TokenKind::Plain,
                        r"👋🔡🌚🌝🌚🌚🌝🌚🌚🌚
👋🔡🌚🌝🌝🌚🌚🌝🌚🌝
👋🔡🌚🌝🌝🌚🌝🌝🌚🌚
👋🔡🌚🌝🌝🌚🌝🌝🌚🌚
👋🔡🌚🌝🌝🌚🌝🌝🌝🌝
👋🔡🌚🌚🌝🌚🌚🌚🌚🌚
👋🔡🌚🌝🌚🌝🌚🌝🌝🌝
👋🔡🌚🌝🌝🌚🌝🌝🌝🌝
👋🔡🌚🌝🌝🌝🌚🌚🌝🌚
👋🔡🌚🌝🌝🌚🌝🌝🌚🌚
👋🔡🌚🌝🌝🌚🌚🌝🌚🌚",
                    )],
                )
            },
        }],
    };
