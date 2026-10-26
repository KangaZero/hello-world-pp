//! Hello-world snippet for `ProgrammingLanguageName::Whitespace`.
//!
//! Imported from leachim6/hello-world (`w/Whitespace.ws` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/Whitespace.ws>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

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
