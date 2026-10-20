//! Hello-world snippet for `ProgrammingLanguageName::Etomemoteetomemoteetomemoteetomemote`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#EtomemoteetomemoteetomemoteetomemotE>
//! - <https://esolangs.org/wiki/EtomemoteetomemoteetomemoteetomemotE>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Etomemoteetomemoteetomemoteetomemote => "EtomemoteetomemoteetomemoteetomemotE",
    versions: [Etomemoteetomemoteetomemoteetomemote => "EtomemoteetomemoteetomemoteetomemotE"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Etomemoteetomemoteetomemoteetomemote,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Etomemoteetomemoteetomemoteetomemote,
            syntax: r"clear;
move forwards; 
flip bit; 
move forwards; 
move forwards;  
move forwards; 
flip bit; 
move forwards; 
move forwards; 
move forwards; 
move forwards; 
flip copy; 
move forwards; 
move forwards; 
move forwards; 
flip bit; 
move forwards; 
flip bit; 
move forwards;
move forwards; 
flip copy; 
move forwards; 
move forwards; 
flip copy; 
move forwards; 
move forwards; 
move forwards; 
move forwards;
move forwards; 
flip copy; 
move forwards; 
move forwards; 
flip copy; 
move forwards; 
move forwards; 
move forwards;  
move forwards;
move forwards; 
flip copy; 
move forwards; 
move forwards; 
flip copy; 
move forwards; 
flip bit; 
move forwards; 
flip bit; 
move forwards; 
flip bit; 
move forwards;
move forwards; 
move forwards; 
flip bit; 
move forwards; 
move forwards; 
move forwards; 
move forwards; 
move forwards;
move forwards; 
flip bit; 
move forwards; 
flip bit; 
move forwards; 
flip copy; 
move forwards; 
flip bit; 
move forwards;
move forwards; 
flip copy; 
move forwards; 
move forwards; 
move forwards; 
flip copy; 
move forwards; 
flip bit; 
move forwards; 
flip bit; 
move forwards; 
flip bit; 
move forwards;
move forwards; 
flip copy; 
move forwards; 
flip bit; 
move forwards; 
move forwards; 
move forwards; 
flip bit; 
move forwards; 
move forwards;
move forwards; 
flip bit; 
move forwards; 
move forwards; 
move forwards; 
flip bit; 
move forwards; 
move forwards; 
move forwards; 
move forwards;
move forwards; 
flip copy; 
move forwards; 
move forwards; 
move forwards; 
move forwards; 
flip bit; 
move forwards; 
move forwards;
print;
end;",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"clear;
move forwards; 
flip bit; 
move forwards; 
move forwards;  
move forwards; 
flip bit; 
move forwards; 
move forwards; 
move forwards; 
move forwards; 
flip copy; 
move forwards; 
move forwards; 
move forwards; 
flip bit; 
move forwards; 
flip bit; 
move forwards;
move forwards; 
flip copy; 
move forwards; 
move forwards; 
flip copy; 
move forwards; 
move forwards; 
move forwards; 
move forwards;
move forwards; 
flip copy; 
move forwards; 
move forwards; 
flip copy; 
move forwards; 
move forwards; 
move forwards;  
move forwards;
move forwards; 
flip copy; 
move forwards; 
move forwards; 
flip copy; 
move forwards; 
flip bit; 
move forwards; 
flip bit; 
move forwards; 
flip bit; 
move forwards;
move forwards; 
move forwards; 
flip bit; 
move forwards; 
move forwards; 
move forwards; 
move forwards; 
move forwards;
move forwards; 
flip bit; 
move forwards; 
flip bit; 
move forwards; 
flip copy; 
move forwards; 
flip bit; 
move forwards;
move forwards; 
flip copy; 
move forwards; 
move forwards; 
move forwards; 
flip copy; 
move forwards; 
flip bit; 
move forwards; 
flip bit; 
move forwards; 
flip bit; 
move forwards;
move forwards; 
flip copy; 
move forwards; 
flip bit; 
move forwards; 
move forwards; 
move forwards; 
flip bit; 
move forwards; 
move forwards;
move forwards; 
flip bit; 
move forwards; 
move forwards; 
move forwards; 
flip bit; 
move forwards; 
move forwards; 
move forwards; 
move forwards;
move forwards; 
flip copy; 
move forwards; 
move forwards; 
move forwards; 
move forwards; 
flip bit; 
move forwards; 
move forwards;
print;
end;",
            )],
        }],
    };
