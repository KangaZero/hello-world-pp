//! Hello-world snippet for `ProgrammingLanguageName::Paintfuck`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Paintfuck>
//! - <https://esolangs.org/wiki/Paintfuck>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Paintfuck => "Paintfuck",
    versions: [Paintfuck => "Paintfuck"],
    extensions: [],
}

const PAINTFUCK: &str = r"*s*s*s*s*enn*enn*s*s*s*s*ee*n*n*n*n*e*e*ssw
*ss*e*ee*e*e*wwn*n*n*n*eeee*s*s*s*s*e*e*een
*n*n*en*es*s*s*sw*eee*sw*wwwwwn[ww]seess*s*
s*s*se*e*n*n*n*n*ee*s*s*s*sw*eeee*wn*n*n*ne
*es*s*s*ees*n*n*n*n*e*es*ws*es*s*ee*n*n*n*n
*[s]ne*e*ee*n*n*n*n*e*es*s*s*sw*eee*nn*n*n*";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Paintfuck,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Paintfuck,
            syntax: PAINTFUCK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PAINTFUCK,
                    [(
                        TokenKind::Plain,
                        r"*s*s*s*s*enn*enn*s*s*s*s*ee*n*n*n*n*e*e*ssw
*ss*e*ee*e*e*wwn*n*n*n*eeee*s*s*s*s*e*e*een
*n*n*en*es*s*s*sw*eee*sw*wwwwwn[ww]seess*s*
s*s*se*e*n*n*n*n*ee*s*s*s*sw*eeee*wn*n*n*ne
*es*s*s*ees*n*n*n*n*e*es*ws*es*s*ee*n*n*n*n
*[s]ne*e*ee*n*n*n*n*e*es*s*s*sw*eee*nn*n*n*",
                    )],
                )
            },
        }],
    };
