//! Hello-world snippet for `ProgrammingLanguageName::MonotonicBoolfuck`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Monotonic_Boolfuck>
//! - <https://esolangs.org/wiki/Monotonic_Boolfuck>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: MonotonicBoolfuck => "Monotonic Boolfuck",
    versions: [MonotonicBoolfuck => "Monotonic Boolfuck"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MonotonicBoolfuck,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MonotonicBoolfuck,
            syntax: r"1
>;;;<;>;;<;;;  H
<;>;<;>;;<;;;; e
;;<;;>;<;;;;   l
;;<;;>;<;;;;   l
<;;;;>;<;;;;   o
;;<;;>;<;;;
;;;;;<;;;
<;;;>;<;;;;;   W
<;;;;>;<;;;;   o
;<;>;;<;;;;;   r
;;<;;>;<;;;;   l
;;<;>;;<;;;;   d
<;>;;;;<;;;",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"1
>;;;<;>;;<;;;  H
<;>;<;>;;<;;;; e
;;<;;>;<;;;;   l
;;<;;>;<;;;;   l
<;;;;>;<;;;;   o
;;<;;>;<;;;
;;;;;<;;;
<;;;>;<;;;;;   W
<;;;;>;<;;;;   o
;<;>;;<;;;;;   r
;;<;;>;<;;;;   l
;;<;>;;<;;;;   d
<;>;;;;<;;;",
            )],
        }],
    };
