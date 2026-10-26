//! Hello-world snippet for `ProgrammingLanguageName::Befunk`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Befunk>
//! - <https://esolangs.org/wiki/Befunk>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Befunk => "Befunk",
    versions: [Befunk => "Befunk"],
    extensions: [],
}

const BEFUNK: &str = r"[code]x = 0, y = 0, rule = LifeColorful
BQMDLROWFCO2LEHQJPNDIX!
[[ COLOR 0 255 255 255 ]]
[[ COLOR 2 0 0 0 ]]
[[ COLOR 3 0 144 144 ]]
[[ COLOR 4 171 0 0 ]]
[[ COLOR 5 171 0 251 ]]
[[ COLOR 6 0 243 192 ]]
[[ COLOR 8 0 127 192 ]]
[[ COLOR 9 0 255 144 ]]
[[ COLOR 10 0 255 0 ]]
[[ COLOR 12 171 0 128 ]]
[[ COLOR 13 0 243 243 ]]
[[ COLOR 14 192 0 0 ]]
[[ COLOR 15 171 171 171 ]]
[[ COLOR 16 171 127 255 ]]
[[ COLOR 17 192 255 216 ]]
[[ COLOR 18 171 171 144 ]]
[[ COLOR 23 171 171 99 ]]
[[ COLOR 24 249 99 249 ]][/code]";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Befunk,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Befunk,
            syntax: BEFUNK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BEFUNK,
                    [(
                        TokenKind::Plain,
                        r"[code]x = 0, y = 0, rule = LifeColorful
BQMDLROWFCO2LEHQJPNDIX!
[[ COLOR 0 255 255 255 ]]
[[ COLOR 2 0 0 0 ]]
[[ COLOR 3 0 144 144 ]]
[[ COLOR 4 171 0 0 ]]
[[ COLOR 5 171 0 251 ]]
[[ COLOR 6 0 243 192 ]]
[[ COLOR 8 0 127 192 ]]
[[ COLOR 9 0 255 144 ]]
[[ COLOR 10 0 255 0 ]]
[[ COLOR 12 171 0 128 ]]
[[ COLOR 13 0 243 243 ]]
[[ COLOR 14 192 0 0 ]]
[[ COLOR 15 171 171 171 ]]
[[ COLOR 16 171 127 255 ]]
[[ COLOR 17 192 255 216 ]]
[[ COLOR 18 171 171 144 ]]
[[ COLOR 23 171 171 99 ]]
[[ COLOR 24 249 99 249 ]][/code]",
                    )],
                )
            },
        }],
    };
