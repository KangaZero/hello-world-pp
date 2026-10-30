//! Hello-world snippet for `ProgrammingLanguageName::Whitespace`.
//!
//! Imported from leachim6/hello-world (`w/Whitespace.ws` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/Whitespace.ws>
//! - <https://esolangs.org/wiki/Whitespace> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Whitespace => "Whitespace",
    versions: [Whitespace => "Whitespace"],
    extensions: [Ws => "ws"],
}

const WHITESPACE: &str = r"   	  	   
	
     		  	 	
	
     		 		  
	
     		 		  
	
     		 				
	
      	     
	
     	 	 			
	
     		 				
	
     			  	 
	
     		 		  
	
     		  	  
	
  

";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Whitespace,
        extensions: &[FileExtension::Ws],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Whitespace,
            syntax: WHITESPACE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WHITESPACE,
                    [(
                        TokenKind::Plain,
                        r"   	  	   
	
     		  	 	
	
     		 		  
	
     		 		  
	
     		 				
	
      	     
	
     	 	 			
	
     		 				
	
     			  	 
	
     		 		  
	
     		  	  
	
  

",
                    )],
                )
            },
        }],
    };
