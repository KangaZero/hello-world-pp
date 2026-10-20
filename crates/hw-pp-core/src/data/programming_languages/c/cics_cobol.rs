//! Hello-world snippet for `ProgrammingLanguageName::CicsCobol`.
//!
//! Imported from the Hello World Collection (`files/CICS-COBOL.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/CICS-COBOL.htm>
//! - <http://helloworldcollection.de/#CICS-COBOL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: CicsCobol => "CICS-COBOL",
    versions: [CicsCobol => "CICS-COBOL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CicsCobol,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CicsCobol,
            syntax: concat!(
                r"-- ",
                output_for_hello_world!(),
                r" in CICS COBOL

000100        IDENTIFICATION DIVISION.                           
000200        PROGRAM-ID. HELLO.                                 
000300       * ",
                output_for_hello_world!(),
                r#" IN CICS COBOL.                        
000400        AUTHOR. ROBERT GOSLING.                            
000500        ENVIRONMENT DIVISION.                              
000600        DATA DIVISION.                                     
000700        WORKING-STORAGE SECTION.                           
000800        01 WS-DATA-AREA PIC X(80) VALUE ""#,
                output_for_hello_world!(),
                r#"".    
000900        PROCEDURE DIVISION.                                
001000            EXEC CICS SEND FROM (WS-DATA-AREA) END-EXEC.   
001100            EXEC CICS RETURN END-EXEC."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"-- ",
                    output_for_hello_world!(),
                    r" in CICS COBOL

000100        IDENTIFICATION DIVISION.                           
000200        PROGRAM-ID. HELLO.                                 
000300       * ",
                    output_for_hello_world!(),
                    r#" IN CICS COBOL.                        
000400        AUTHOR. ROBERT GOSLING.                            
000500        ENVIRONMENT DIVISION.                              
000600        DATA DIVISION.                                     
000700        WORKING-STORAGE SECTION.                           
000800        01 WS-DATA-AREA PIC X(80) VALUE ""#,
                    output_for_hello_world!(),
                    r#"".    
000900        PROCEDURE DIVISION.                                
001000            EXEC CICS SEND FROM (WS-DATA-AREA) END-EXEC.   
001100            EXEC CICS RETURN END-EXEC."#
                ),
            )],
        }],
    };
