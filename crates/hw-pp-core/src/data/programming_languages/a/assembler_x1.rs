//! Hello-world snippet for `ProgrammingLanguageName::AssemblerX1`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(X1).htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28X1%29.htm>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(X1)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: AssemblerX1 => "Assembler (X1)",
    versions: [AssemblerX1 => "Assembler (X1)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerX1,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerX1,
            syntax: r"                  dp zz 0 x 5

                  da 0 zz 0   di
	      0   2b 1       a		       set address increment to 1
	 3->  1   2a 4 zz 0  c p	       load next character to A
	      2   6y 2 xp    		       print it
	      3 y 2t 1 zz 0  a		       loop if not last char
	      4   7p   	  		       and halt
	      5 dn + 19			       lower case
	      6	  + 28			       h
	      7	  + 25			       e
	      8	  + 32			       l
	      9	  + 32			       l
	     10	  + 35			       o
	     11	  + 15			       comma
	     12	  + 20			       space
	     13   + 43			       w
	     14	  + 35			       o
	     15	  + 38			       r
	     16	  + 32			       l
	     17	  + 24			       d
	     18	  - 52			       clrf, negative = last char

                  de 0 zz 0",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"                  dp zz 0 x 5

                  da 0 zz 0   di
	      0   2b 1       a		       set address increment to 1
	 3->  1   2a 4 zz 0  c p	       load next character to A
	      2   6y 2 xp    		       print it
	      3 y 2t 1 zz 0  a		       loop if not last char
	      4   7p   	  		       and halt
	      5 dn + 19			       lower case
	      6	  + 28			       h
	      7	  + 25			       e
	      8	  + 32			       l
	      9	  + 32			       l
	     10	  + 35			       o
	     11	  + 15			       comma
	     12	  + 20			       space
	     13   + 43			       w
	     14	  + 35			       o
	     15	  + 38			       r
	     16	  + 32			       l
	     17	  + 24			       d
	     18	  - 52			       clrf, negative = last char

                  de 0 zz 0",
            )],
        }],
    };
