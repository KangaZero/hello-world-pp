//! Hello-world snippet for `ProgrammingLanguageName::Toadskin`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Toadskin>
//! - <https://esolangs.org/wiki/Toadskin>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Toadskin => "Toadskin",
    versions: [Toadskin => "Toadskin"],
    extensions: [],
}

const TOADSKIN: &str = r":V+++++;:XVV;:v-----;:xvv;XXXXXXX++.<XXXXXXXXXX+.V
++..+++.<XXX++.>>XV.XX++++.+++.v-.x++.<XXX+++.<X.>";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Toadskin,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Toadskin,
            syntax: TOADSKIN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TOADSKIN,
                    [(
                        TokenKind::Plain,
                        r":V+++++;:XVV;:v-----;:xvv;XXXXXXX++.<XXXXXXXXXX+.V
++..+++.<XXX++.>>XV.XX++++.+++.v-.x++.<XXX+++.<X.>",
                    )],
                )
            },
        }],
    };
