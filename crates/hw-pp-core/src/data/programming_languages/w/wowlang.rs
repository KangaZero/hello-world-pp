//! Hello-world snippet for `ProgrammingLanguageName::Wowlang`.
//!
//! Imported from leachim6/hello-world (`w/wowLang.wow` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/wowLang.wow>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Wowlang => "wowLang",
    versions: [Wowlang => "wowLang"],
    extensions: [Wow => "wow"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wowlang,
        extensions: &[FileExtension::Wow],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wowlang,
            syntax: r"wow WOw wow wow wow wow wow wow wow woW WOW Wow wOW woW wow WoW wow WOw wow woW WOW WOW WOW WOW WOW Wow wOW woW WOW WOW WoW WOW WOW WOW WOW WOW WOW WOW WoW WoW WOW WOW WOW WoW WOw wow wow wow woW WOW Wow wOW woW wow wow wow wow wow WoW wow wow wow WOw wow woW WOW WOW WOW Wow wOW woW WoW wow WOw wow wow wow woW WOW Wow wOW woW wow wow wow WoW WOW WOW WOW WoW wow wow wow wow wow wow WoW wow wow wow wow wow wow wow wow WoW ",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"wow WOw wow wow wow wow wow wow wow woW WOW Wow wOW woW wow WoW wow WOw wow woW WOW WOW WOW WOW WOW Wow wOW woW WOW WOW WoW WOW WOW WOW WOW WOW WOW WOW WoW WoW WOW WOW WOW WoW WOw wow wow wow woW WOW Wow wOW woW wow wow wow wow wow WoW wow wow wow WOw wow woW WOW WOW WOW Wow wOW woW WoW wow WOw wow wow wow woW WOW Wow wOW woW wow wow wow WoW WOW WOW WOW WoW wow wow wow wow wow wow WoW wow wow wow wow wow wow wow wow WoW ",
            )],
        }],
    };
