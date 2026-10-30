//! Hello-world snippet for `ProgrammingLanguageName::Linguine`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Linguine>
//! - <https://esolangs.org/wiki/Linguine>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Linguine => "Linguine",
    versions: [Linguine => "Linguine"],
    extensions: [],
}

const LINGUINE: &str = r" 1[0=72,0$,0+29,0$,0+7,0$,0$,0+3,0$,1=32,1$,0-24,0$,0+24,0$,0+3,0$,0-6,0$,0-8,0$,1+1,1$,1-23,1$]0";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Linguine,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Linguine,
            syntax: LINGUINE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LINGUINE,
                    [(
                        TokenKind::Plain,
                        r" 1[0=72,0$,0+29,0$,0+7,0$,0$,0+3,0$,1=32,1$,0-24,0$,0+24,0$,0+3,0$,0-6,0$,0-8,0$,1+1,1$,1-23,1$]0",
                    )],
                )
            },
        }],
    };
