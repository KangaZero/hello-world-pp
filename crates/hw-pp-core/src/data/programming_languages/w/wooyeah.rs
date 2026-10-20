//! Hello-world snippet for `ProgrammingLanguageName::Wooyeah`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#WooYeah>
//! - <https://esolangs.org/wiki/WooYeah>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Wooyeah => "WooYeah",
    versions: [Wooyeah => "WooYeah"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wooyeah,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wooyeah,
            syntax: concat!(
                r"g                                                                                                                                                                                         
{",
                output_for_hello_world!(),
                r" program}                                                                                                                                                                     
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah ^                                                                                                                         
wooyeah wooyeah wooyeah wooyeah wooyeah v                                                                                                                                                 
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah v                                                                                         
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah v                                                                                         
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah v                                                                 
cm                                                                                                                                                                                        
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah  
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah ^ 
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah 
wooyeah v                                         
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah v                                                                                         
wooyeah wooyeah wooyeah wooyeah v                                                                                                                                                         
em                                                                                                                                                                                        
s"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"g                                                                                                                                                                                         
{",
                    output_for_hello_world!(),
                    r" program}                                                                                                                                                                     
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah ^                                                                                                                         
wooyeah wooyeah wooyeah wooyeah wooyeah v                                                                                                                                                 
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah v                                                                                         
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah v                                                                                         
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah v                                                                 
cm                                                                                                                                                                                        
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah  
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah ^ 
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah 
wooyeah v                                         
wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah wooyeah v                                                                                         
wooyeah wooyeah wooyeah wooyeah v                                                                                                                                                         
em                                                                                                                                                                                        
s"
                ),
            )],
        }],
    };
