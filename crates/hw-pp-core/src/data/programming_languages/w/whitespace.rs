//! Hello-world snippet for `ProgrammingLanguageName::Whitespace`.
//!
//! Imported from leachim6/hello-world (`w/Whitespace.ws` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Whitespace => "Whitespace",
    versions: [Whitespace => "Whitespace"],
    extensions: [Ws => "ws"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Whitespace,
        extensions: &[FileExtension::Ws],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Whitespace,
            syntax: r"   	  	   
	
     		  	 	
	
     		 		  
	
     		 		  
	
     		 				
	
      	     
	
     	 	 			
	
     		 				
	
     			  	 
	
     		 		  
	
     		  	  
	
  

",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"   	  	   
	
     		  	 	
	
     		 		  
	
     		 		  
	
     		 				
	
      	     
	
     	 	 			
	
     		 				
	
     			  	 
	
     		 		  
	
     		  	  
	
  

",
            )],
        }],
    };
