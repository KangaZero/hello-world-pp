//! Hello-world snippet for `ProgrammingLanguageName::N4kowoLikeEsolang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#4KOWO-like_esolang>
//! - <https://esolangs.org/wiki/4KOWO-like_esolang>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: N4kowoLikeEsolang => "4KOWO-like esolang",
    versions: [N4kowoLikeEsolang => "4KOWO-like esolang"],
    extensions: [],
}

const N4KOWO_LIKE_ESOLANG: &str = concat!(r"⎵", output_for_hello_world!(), r" ⏎ ⎋ ⚐");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N4kowoLikeEsolang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N4kowoLikeEsolang,
            syntax: N4KOWO_LIKE_ESOLANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N4KOWO_LIKE_ESOLANG,
                    [(
                        TokenKind::Plain,
                        concat!(r"⎵", output_for_hello_world!(), r" ⏎ ⎋ ⚐"),
                    )],
                )
            },
        }],
    };
