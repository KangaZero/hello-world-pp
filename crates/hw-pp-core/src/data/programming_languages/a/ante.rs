//! Hello-world snippet for `ProgrammingLanguageName::Ante`.
//!
//! Imported from leachim6/hello-world (`a/Ante.ante` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Ante.ante>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Ante => "Ante",
    versions: [Ante => "Ante"],
    extensions: [Ante => "ante"],
}

const ANTE: &str =
    r"9♦8♥J♦A♦2♣3♥7♠J♦A♦7♦J♦J♦A♦3♦J♦5♥6♦4♥J♥A♥6♠6♠J♥A♦8♦J♦A♦8♠J♦A♦3♦J♦A♦6♠J♦A♦8♠J♦A♥3♦2♠J♥A♥2♣6♠J♥";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ante,
        extensions: &[FileExtension::Ante],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ante,
            syntax: ANTE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ANTE,
                    [(
                        TokenKind::Plain,
                        r"9♦8♥J♦A♦2♣3♥7♠J♦A♦7♦J♦J♦A♦3♦J♦5♥6♦4♥J♥A♥6♠6♠J♥A♦8♦J♦A♦8♠J♦A♦3♦J♦A♦6♠J♦A♦8♠J♦A♥3♦2♠J♥A♥2♣6♠J♥",
                    )],
                )
            },
        }],
    };
