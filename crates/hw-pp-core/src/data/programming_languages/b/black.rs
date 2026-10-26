//! Hello-world snippet for `ProgrammingLanguageName::Black`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Black>
//! - <https://esolangs.org/wiki/Black>

use crate::data::programming_languages::ProgrammingParadigm::Reflective;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Black => "Black",
    versions: [Black => "Black"],
    extensions: [],
}

const BLACK: &str = r"           # #  

                 #
       #         

              #


     #    #    # 
                
           #    
              7
              2
              N
              1
              0
              1
              N
              1
              0
              8
              N
              1
              0
              8
              N
              1
              1
              1
              N
              4
              4
              N
              3
              2
              N
              1
              1
              9
              N
              1
              1
              1
              N
              1
              1
              4
              N
              1
              0
              8
              N
              1
              0
              0
              N
              3
              3
              N
              1
              0
              N
              ##";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Black,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Black,
            syntax: BLACK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BLACK,
                    [(
                        TokenKind::Plain,
                        r"           # #  

                 #
       #         

              #


     #    #    # 
                
           #    
              7
              2
              N
              1
              0
              1
              N
              1
              0
              8
              N
              1
              0
              8
              N
              1
              1
              1
              N
              4
              4
              N
              3
              2
              N
              1
              1
              9
              N
              1
              1
              1
              N
              1
              1
              4
              N
              1
              0
              8
              N
              1
              0
              0
              N
              3
              3
              N
              1
              0
              N
              ##",
                    )],
                )
            },
        }],
    };
