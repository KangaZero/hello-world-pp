//! Hello-world snippet for `ProgrammingLanguageName::Emojicode`.
//!
//! Imported from leachim6/hello-world (`e/Emojicode.emojic` at `a152253`),
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
    language: Emojicode => "Emojicode",
    versions: [Emojicode => "Emojicode"],
    extensions: [Emojic => "emojic"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Emojicode,
        extensions: &[FileExtension::Emojic],
        categories: &[],
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
