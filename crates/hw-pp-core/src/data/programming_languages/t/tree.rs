//! Hello-world snippet for `ProgrammingLanguageName::Tree`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Tree>
//! - <https://esolangs.org/wiki/Tree>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Tree => "Tree",
    versions: [Tree => "Tree"],
    extensions: [],
}

const TREE: &str = r"      ^
    ^^|^^ 
   ^^\|/^^
   H^ |/^^^
     \| e
      |/
   ol~|  
     \| 
  , 32|  W
   \/ | /      
    \ |/   
 l   \|  
 \  d |   o
  \/ !|r /   
   \/ |\/
    \ |/ 
     \|";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tree,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tree,
            syntax: TREE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TREE,
                    [(
                        TokenKind::Plain,
                        r"      ^
    ^^|^^ 
   ^^\|/^^
   H^ |/^^^
     \| e
      |/
   ol~|  
     \| 
  , 32|  W
   \/ | /      
    \ |/   
 l   \|  
 \  d |   o
  \/ !|r /   
   \/ |\/
    \ |/ 
     \|",
                    )],
                )
            },
        }],
    };
