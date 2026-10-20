//! Hello-world snippet for `ProgrammingLanguageName::Masl`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#MASL>
//! - <https://esolangs.org/wiki/MASL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Masl => "MASL",
    versions: [Masl => "MASL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Masl,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Masl,
            syntax: concat!(
                r" # ",
                output_for_hello_world!(),
                r" in MASL v1.0.0
 psh 0,0,0x48
 psh 0,1,0x65
 psh 0,2,0x6c
 psh 0,3,0x6f
 
 psh 1,0,0x57
 psh 1,1,0x72
 psh 1,2,0x64
 
 psh 2,0,0x2c
 psh 2,1,0x21
 
 cnt 0,0,ch
 cnt 0,1,ch
 cnt 0,2,ch
 cnt 0,2,ch
 cnt 0,3,ch
 
 cnt 2,0,ch
 cnt 3,3,ch
 
 cnt 1,0,ch
 cnt 0,3,ch
 cnt 1,1,ch
 cnt 0,2,ch
 cnt 1,2,ch
 
 cnt 2,1,ch"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r" # ",
                    output_for_hello_world!(),
                    r" in MASL v1.0.0
 psh 0,0,0x48
 psh 0,1,0x65
 psh 0,2,0x6c
 psh 0,3,0x6f
 
 psh 1,0,0x57
 psh 1,1,0x72
 psh 1,2,0x64
 
 psh 2,0,0x2c
 psh 2,1,0x21
 
 cnt 0,0,ch
 cnt 0,1,ch
 cnt 0,2,ch
 cnt 0,2,ch
 cnt 0,3,ch
 
 cnt 2,0,ch
 cnt 3,3,ch
 
 cnt 1,0,ch
 cnt 0,3,ch
 cnt 1,1,ch
 cnt 0,2,ch
 cnt 1,2,ch
 
 cnt 2,1,ch"
                ),
            )],
        }],
    };
