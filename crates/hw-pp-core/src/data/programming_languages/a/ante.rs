//! Hello-world snippet for `ProgrammingLanguageName::Ante`.
//!
//! Imported from leachim6/hello-world (`a/Ante.ante` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Ante => "Ante",
    versions: [Ante => "Ante"],
    extensions: [Ante => "ante"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ante,
        extensions: &[FileExtension::Ante],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ante,
            syntax: r"9♦8♥J♦A♦2♣3♥7♠J♦A♦7♦J♦J♦A♦3♦J♦5♥6♦4♥J♥A♥6♠6♠J♥A♦8♦J♦A♦8♠J♦A♦3♦J♦A♦6♠J♦A♦8♠J♦A♥3♦2♠J♥A♥2♣6♠J♥",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"9♦8♥J♦A♦2♣3♥7♠J♦A♦7♦J♦J♦A♦3♦J♦5♥6♦4♥J♥A♥6♠6♠J♥A♦8♦J♦A♦8♠J♦A♦3♦J♦A♦6♠J♦A♦8♠J♦A♥3♦2♠J♥A♥2♣6♠J♥",
            )],
        }],
    };
