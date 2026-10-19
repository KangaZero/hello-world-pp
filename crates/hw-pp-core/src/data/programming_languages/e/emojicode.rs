//! Hello-world snippet for `ProgrammingLanguageName::Emojicode`.
//!
//! Imported from leachim6/hello-world (`e/Emojicode.emojic` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Emojicode.emojic>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Emojicode => "Emojicode",
    versions: [Emojicode => "Emojicode"],
    extensions: [Emojic => "emojic"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Emojicode,
        extensions: &[FileExtension::Emojic],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Emojicode,
            syntax: concat!(
                r"🏁 🍇
  😀 🔤",
                output_for_hello_world!(),
                r"🔤❗️
🍉"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"🏁 🍇
  😀 🔤",
                    output_for_hello_world!(),
                    r"🔤❗️
🍉"
                ),
            )],
        }],
    };
