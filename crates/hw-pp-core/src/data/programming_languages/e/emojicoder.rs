//! Hello-world snippet for `ProgrammingLanguageName::Emojicoder`.
//!
//! Imported from leachim6/hello-world (`e/EmojiCoder` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Emojicoder => "EmojiCoder",
    versions: [Emojicoder => "EmojiCoder"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Emojicoder,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Emojicoder,
            syntax: r"👋🔡🌚🌝🌚🌚🌝🌚🌚🌚
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
            tokenized_syntax: &[(
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
        }],
    };
