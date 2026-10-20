//! Hello-world snippet for `ProgrammingLanguageName::Ziim`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Ziim>
//! - <https://esolangs.org/wiki/Ziim>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Ziim => "Ziim",
    versions: [Ziim => "Ziim"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ziim,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ziim,
            syntax: r"    ↘↓                        
        ↘                     
↘          ←              ↙   
          ⤢↘           ↘← ↑   
 ↘←     ↑⤡↙ → ↙         ↓↙↙   
      ↙  → ↗           ↙      
 ↘ ←  ↙↙↘     ↙↘ ↙     ↑   ↘← 
↘     ↑↕ ←     ↓↓↑      ↙   ← 
  ↘   ←  ↙ ↕← ↗            ↔↙ 
     ↗↓ ↑↑  →↖  ↙←  ↓   ↘     
 →    ↘ ↘   ↓↘← ↗     ↙↙      
   ↙    ↖ ↘  ←↓↙←     ↑  ↑↑   
↑ →         ↘  →          ↙   
     ↘        ↙      ←↙       
  ↘  ↕     ↔       ↙↙     ←  ↙
                  ⤡           
    ↑ ↘  ←↓↓↓ ↓↓↓   ↙↘        
        ↘→  ↘ ↙  ←    ↑       
   ↓  ↘  → ↘   ↙ ←            
         →↘ ↓ ↓ ↙←   ↕←       
       ↗   →↘ ↙←              
         →↗  ↓  ↖←   ↖ ↕ ↔  ↙ 
    ↗↔ ↖  ↑↓   ↙↑ ↖←       ↑↘ 
    ↙       ↓ →↗→↖↑ ↑        ↑
           ↗↖  ↑→ ↙↓ ↙        
  ↑    ↗      ↔   ↖↙←↙↓↖      
  ↗↔  ↕ ↑  →              ↙   
  ↘  ←↗ →↖ ↘  ↕  ←    ↖   ⤡   
  ↗         ←        ↙  ↑     
      ↙       ↖   ↓↖←     ↖ ↑ 
   ↓ ↓↑↙        → ↘↑↓↖        
   ↗ ↙←    ↑      ↙→     ↕    
  →                   ↖       
   ⤡    ↖       →↗ ↓     ↖    
    ↑            ↑ ↗↖         ",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"    ↘↓                        
        ↘                     
↘          ←              ↙   
          ⤢↘           ↘← ↑   
 ↘←     ↑⤡↙ → ↙         ↓↙↙   
      ↙  → ↗           ↙      
 ↘ ←  ↙↙↘     ↙↘ ↙     ↑   ↘← 
↘     ↑↕ ←     ↓↓↑      ↙   ← 
  ↘   ←  ↙ ↕← ↗            ↔↙ 
     ↗↓ ↑↑  →↖  ↙←  ↓   ↘     
 →    ↘ ↘   ↓↘← ↗     ↙↙      
   ↙    ↖ ↘  ←↓↙←     ↑  ↑↑   
↑ →         ↘  →          ↙   
     ↘        ↙      ←↙       
  ↘  ↕     ↔       ↙↙     ←  ↙
                  ⤡           
    ↑ ↘  ←↓↓↓ ↓↓↓   ↙↘        
        ↘→  ↘ ↙  ←    ↑       
   ↓  ↘  → ↘   ↙ ←            
         →↘ ↓ ↓ ↙←   ↕←       
       ↗   →↘ ↙←              
         →↗  ↓  ↖←   ↖ ↕ ↔  ↙ 
    ↗↔ ↖  ↑↓   ↙↑ ↖←       ↑↘ 
    ↙       ↓ →↗→↖↑ ↑        ↑
           ↗↖  ↑→ ↙↓ ↙        
  ↑    ↗      ↔   ↖↙←↙↓↖      
  ↗↔  ↕ ↑  →              ↙   
  ↘  ←↗ →↖ ↘  ↕  ←    ↖   ⤡   
  ↗         ←        ↙  ↑     
      ↙       ↖   ↓↖←     ↖ ↑ 
   ↓ ↓↑↙        → ↘↑↓↖        
   ↗ ↙←    ↑      ↙→     ↕    
  →                   ↖       
   ⤡    ↖       →↗ ↓     ↖    
    ↑            ↑ ↗↖         ",
            )],
        }],
    };
