//! Hello-world snippet for `ProgrammingLanguageName::Emojicode`.
//!
//! Imported from leachim6/hello-world (`e/Emojicode.emojic` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Emojicode.emojic>
//! - <https://esolangs.org/wiki/Emojicode> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Emojicode => "Emojicode",
    versions: [Emojicode => "Emojicode"],
    extensions: [Emojic => "emojic"],
}

const EMOJICODE: &str = concat!(
    r"🏁 🍇
  😀 🔤",
    output_for_hello_world!(),
    r"🔤❗️
🍉"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Emojicode,
        extensions: &[FileExtension::Emojic],
        categories: &[Esolang],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Emojicode,
            syntax: EMOJICODE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EMOJICODE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"🏁 🍇
  😀 🔤",
                            output_for_hello_world!(),
                            r"🔤❗️
🍉"
                        ),
                    )],
                )
            },
        }],
    };
