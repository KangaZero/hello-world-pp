//! Hello-world snippet for `ProgrammingLanguageName::HanoiLove`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Hanoi_Love>
//! - <https://esolangs.org/wiki/Hanoi_Love>

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: HanoiLove => "Hanoi Love",
    versions: [HanoiLove => "Hanoi Love"],
    extensions: [],
}

const HANOI_LOVE: &str = r#".'...,;';';;';';';'.,...;"'.'...,;;';;';';;'.,...;"';;;;;;;"'"';;;
"'.'.,...'...,;';';';';'.,...;"'..'...,'...,;;';';';'.,...`"'.'...,
;;';';';'.,...;"';;;"'``````"'````````"'.'.,..;"'.'.,..;;;;;;;;;;"'"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::HanoiLove,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::HanoiLove,
            syntax: HANOI_LOVE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HANOI_LOVE,
                    [(
                        TokenKind::Plain,
                        r#".'...,;';';;';';';'.,...;"'.'...,;;';;';';;'.,...;"';;;;;;;"'"';;;
"'.'.,...'...,;';';';';'.,...;"'..'...,'...,;;';';';'.,...`"'.'...,
;;';';';'.,...;"';;;"'``````"'````````"'.'.,..;"'.'.,..;;;;;;;;;;"'"#,
                    )],
                )
            },
        }],
    };
