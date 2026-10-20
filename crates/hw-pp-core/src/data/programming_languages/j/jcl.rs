//! Hello-world snippet for `ProgrammingLanguageName::Jcl`.
//!
//! Imported from the Hello World Collection (`files/JCL.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/JCL.txt>
//! - <http://helloworldcollection.de/#JCL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Jcl => "JCL",
    versions: [Jcl => "JCL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jcl,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jcl,
            syntax: concat!(
                r"//HERIB    JOB  ,'HERIBERT OTTEN',PRTY=12
//* ",
                output_for_hello_world!(),
                r" FOR MVS            
 //HALLO    EXEC PGM=IEBGENER       
 //SYSIN    DD DUMMY                
 //SYSPRINT DD SYSOUT=*             
 //SYSUT2   DD SYSOUT=T             
 //SYSUT1   DD *                    
 ",
                output_for_hello_world!(),
                r"                       
 /*                                 
 //    "
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"//HERIB    JOB  ,'HERIBERT OTTEN',PRTY=12
//* ",
                    output_for_hello_world!(),
                    r" FOR MVS            
 //HALLO    EXEC PGM=IEBGENER       
 //SYSIN    DD DUMMY                
 //SYSPRINT DD SYSOUT=*             
 //SYSUT2   DD SYSOUT=T             
 //SYSUT1   DD *                    
 ",
                    output_for_hello_world!(),
                    r"                       
 /*                                 
 //    "
                ),
            )],
        }],
    };
