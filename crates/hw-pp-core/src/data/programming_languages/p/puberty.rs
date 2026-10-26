//! Hello-world snippet for `ProgrammingLanguageName::Puberty`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#PUBERTY>
//! - <https://esolangs.org/wiki/PUBERTY>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Puberty => "PUBERTY",
    versions: [Puberty => "PUBERTY"],
    extensions: [],
}

const PUBERTY: &str = r"It is June 12, 2018, 03:02:04 PM. Izu is in his bed, bored. His secret kinks are vore, fatfurs, inflation, growth and kitsunes. 
Then he spots fatfurs. Soon the following sounds become audible. 

fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap yes fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap yes yeah yeah yeah yes yes fap fap fap yes yEAH YeaH 
fAtfURs,hNNg yEAH fap fap inflation,hnng yeah fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap vore,hnng 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap yeah fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap vore,hnng fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap yeah ugh fap fap fap fap fap
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fatfurs,hnng yeah fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap vore,hnng yeah squirt ugh 
growth,hnng kitsunes,fuck yes sigh";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Puberty,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Puberty,
            syntax: PUBERTY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PUBERTY,
                    [(
                        TokenKind::Plain,
                        r"It is June 12, 2018, 03:02:04 PM. Izu is in his bed, bored. His secret kinks are vore, fatfurs, inflation, growth and kitsunes. 
Then he spots fatfurs. Soon the following sounds become audible. 

fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap yes fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap yes yeah yeah yeah yes yes fap fap fap yes yEAH YeaH 
fAtfURs,hNNg yEAH fap fap inflation,hnng yeah fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap vore,hnng 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap yeah fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap vore,hnng fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap yeah ugh fap fap fap fap fap
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fatfurs,hnng yeah fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap fap 
fap fap fap fap fap fap fap fap fap vore,hnng yeah squirt ugh 
growth,hnng kitsunes,fuck yes sigh",
                    )],
                )
            },
        }],
    };
