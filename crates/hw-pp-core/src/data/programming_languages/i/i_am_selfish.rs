//! Hello-world snippet for `ProgrammingLanguageName::IAmSelfish`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#I_am_selfish>
//! - <https://esolangs.org/wiki/I_am_selfish>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: IAmSelfish => "I am selfish",
    versions: [IAmSelfish => "I am selfish"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::IAmSelfish,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::IAmSelfish,
            syntax: r"IΙΙIΙΙIΙΙIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIΙΙIΙΙIΙΙIIΙΙIIΙΙIIΙΙIΙΙIΙΙIΙΙIΙΙ
IΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIΙΙ
IΙΙIΙΙIΙΙIΙΙIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIΙΙIIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙ
IΙΙIΙΙIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙII
ΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙIIΙΙIIΙΙIIΙΙIIΙΙII
ΙΙIIΙΙIIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIΙΙIΙΙIIΙΙII",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"IΙΙIΙΙIΙΙIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIΙΙIΙΙIΙΙIIΙΙIIΙΙIIΙΙIΙΙIΙΙIΙΙIΙΙ
IΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIΙΙ
IΙΙIΙΙIΙΙIΙΙIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIΙΙIIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙ
IΙΙIΙΙIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙII
ΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙIIΙΙIIΙΙIIΙΙIIΙΙII
ΙΙIIΙΙIIΙΙIΙΙIΙΙIΙΙIΙΙIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIIΙΙIΙΙIΙΙIIΙΙII",
            )],
        }],
    };
