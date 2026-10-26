//! Hello-world snippet for `ProgrammingLanguageName::FishCode`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Fish_Code>
//! - <https://esolangs.org/wiki/Fish_Code>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: FishCode => "Fish Code",
    versions: [FishCode => "Fish Code"],
    extensions: [],
}

const FISH_CODE: &str = r"<((72)).((101)).((108)).((108)).((111)).((32)).((87)).((111)).((114)).((108)).((100)).((33)).><";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FishCode,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FishCode,
            syntax: FISH_CODE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FISH_CODE,
                    [(
                        TokenKind::Plain,
                        r"<((72)).((101)).((108)).((108)).((111)).((32)).((87)).((111)).((114)).((108)).((100)).((33)).><",
                    )],
                )
            },
        }],
    };
