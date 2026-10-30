//! Hello-world snippet for `ProgrammingLanguageName::Bawkbawk`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Bawkbawk>
//! - <https://esolangs.org/wiki/Bawkbawk>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Bawkbawk => "Bawkbawk",
    versions: [Bawkbawk => "Bawkbawk"],
    extensions: [],
}

const BAWKBAWK: &str = r"feed forage sad happy sad sad happy sad sad sad theeggcamefirst
chirp forage sad theeggcamefirst

feed forage sad happy sad sad sad happy sad happy
theeggcamefirst chirp forage sad theeggcamefirst

feed forage sad happy sad 
sad happy happy sad sad theeggcamefirst chirp chirp forage sad
theeggcamefirst

feed forage happy sad sad happy happy happy happy theeggcamefirst chirp
forage sad theeggcamefirst

feed forage sad sad happy sad happy happy sad
sad theeggcamefirst chirp forage sad
theeggcamefirst

feed forage sad sad happy sad sad sad sad sad
theeggcamefirst chirp forage sad theeggcamefirst

feed forage sad happy sad happy sad happy happy happy
theeggcamefirst chirp forage sad theeggcamefirst

feed forage happy sad sad happy happy happy happy theeggcamefirst chirp
forage sad theeggcamefirst

feed forage sad happy sad happy sad sad happy sad theeggcamefirst chirp
forage sad theeggcamefirst

feed forage sad happy sad 
sad happy happy sad sad theeggcamefirst chirp forage sad
theeggcamefirst

feed forage sad happy sad sad sad happy sad sad theeggcamefirst chirp forage sad
theeggcamefirst";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bawkbawk,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bawkbawk,
            syntax: BAWKBAWK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BAWKBAWK,
                    [(
                        TokenKind::Plain,
                        r"feed forage sad happy sad sad happy sad sad sad theeggcamefirst
chirp forage sad theeggcamefirst

feed forage sad happy sad sad sad happy sad happy
theeggcamefirst chirp forage sad theeggcamefirst

feed forage sad happy sad 
sad happy happy sad sad theeggcamefirst chirp chirp forage sad
theeggcamefirst

feed forage happy sad sad happy happy happy happy theeggcamefirst chirp
forage sad theeggcamefirst

feed forage sad sad happy sad happy happy sad
sad theeggcamefirst chirp forage sad
theeggcamefirst

feed forage sad sad happy sad sad sad sad sad
theeggcamefirst chirp forage sad theeggcamefirst

feed forage sad happy sad happy sad happy happy happy
theeggcamefirst chirp forage sad theeggcamefirst

feed forage happy sad sad happy happy happy happy theeggcamefirst chirp
forage sad theeggcamefirst

feed forage sad happy sad happy sad sad happy sad theeggcamefirst chirp
forage sad theeggcamefirst

feed forage sad happy sad 
sad happy happy sad sad theeggcamefirst chirp forage sad
theeggcamefirst

feed forage sad happy sad sad sad happy sad sad theeggcamefirst chirp forage sad
theeggcamefirst",
                    )],
                )
            },
        }],
    };
