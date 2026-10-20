//! Hello-world snippet for `ProgrammingLanguageName::Whendo`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Whendo>
//! - <https://esolangs.org/wiki/Whendo>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Whendo => "Whendo",
    versions: [Whendo => "Whendo"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Whendo,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Whendo,
            syntax: r"a==0=>a+=1=>72
a==1=>a+=1=>101
a==2=>a+=1=>108
a==3=>a+=1=>108
a==4=>a+=1=>111
a==5=>a+=1=>44
a==6=>a+=1=>32
a==7=>a+=1=>87
a==8=>a+=1=>111
a==9=>a+=1=>114
a==10=>a+=1=>108
a==11=>a+=1=>100
a==12=>a+=1=>33",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"a==0=>a+=1=>72
a==1=>a+=1=>101
a==2=>a+=1=>108
a==3=>a+=1=>108
a==4=>a+=1=>111
a==5=>a+=1=>44
a==6=>a+=1=>32
a==7=>a+=1=>87
a==8=>a+=1=>111
a==9=>a+=1=>114
a==10=>a+=1=>108
a==11=>a+=1=>100
a==12=>a+=1=>33",
            )],
        }],
    };
