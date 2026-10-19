//! Hello-world snippet for `ProgrammingLanguageName::Pairpointing`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Pairpointing>
//! - <https://esolangs.org/wiki/Pairpointing>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Pairpointing => "Pairpointing",
    versions: [Pairpointing => "Pairpointing"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pairpointing,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pairpointing,
            syntax: r"H = 0, 0, 0, 1, 0, 0, 1, 0, ;
e = 1, 0, 1, 0, 0, 1, 1, 0, ;
l = 0, 0, 1, 1, 0, 1, 1, 0, ;
o = 1, 1, 1, 1, 0, 1, 1, 0, ;
c = 0, 0, 1, 1, 0, 1, 0, 0, ;
s = 0, 0, 0, 0, 0, 1, 0, 0, ;
w = 1, 1, 1, 0, 1, 1, 1, 0, ;
r = 0, 1, 0, 0, 1, 1, 1, 0, ;
d = 0, 0, 1, 0, 0, 1, 1, 0, ;
x = 1, 0, 0, 0, 0, 1, 0, 0, ;
output H,e,l,l,o,c,s,w,o,r,l,d,x,;",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"H = 0, 0, 0, 1, 0, 0, 1, 0, ;
e = 1, 0, 1, 0, 0, 1, 1, 0, ;
l = 0, 0, 1, 1, 0, 1, 1, 0, ;
o = 1, 1, 1, 1, 0, 1, 1, 0, ;
c = 0, 0, 1, 1, 0, 1, 0, 0, ;
s = 0, 0, 0, 0, 0, 1, 0, 0, ;
w = 1, 1, 1, 0, 1, 1, 1, 0, ;
r = 0, 1, 0, 0, 1, 1, 1, 0, ;
d = 0, 0, 1, 0, 0, 1, 1, 0, ;
x = 1, 0, 0, 0, 0, 1, 0, 0, ;
output H,e,l,l,o,c,s,w,o,r,l,d,x,;",
            )],
        }],
    };
