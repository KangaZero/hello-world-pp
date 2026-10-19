//! Hello-world snippet for `ProgrammingLanguageName::CIntercal`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#C-INTERCAL>
//! - <https://esolangs.org/wiki/C-INTERCAL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: CIntercal => "C-INTERCAL",
    versions: [CIntercal => "C-INTERCAL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CIntercal,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CIntercal,
            syntax: concat!(
                r#"        PLEASE NOTE THIS PROGRAM RECOGNIZES ""#,
                output_for_hello_world!(),
                r##"" USING COME FROM
        DON'T TYPE IN ANYTHING ELSE, OR YOU'LL GET AN ERROR!
 
        PLEASE NOTE: COMPILE WITH ick -m FOR THIS TO WORK.
        
        DO ,1 <- #12
 (1)    DO WRITE IN ,1
        DO GIVE UP
 
        PLEASE NOTE THIS CHECKS EACH CHARACTER IN TURN
        DO COME FROM '#255~'"?',1SUB#1'$#72"~"#0$#255"''~#1
        PLEASE START WITH AN H NEXT TIME
 
        DO COME FROM '#255~'"?',1SUB#2'$#253"~"#0$#255"''~#1
        DO (2) NEXT
        DO REMEMBER THAT E COMES SECOND
 
        DO COME FROM '#255~'"?',1SUB#3'$#7"~"#0$#255"''~#1
        DO (4) NEXT 
        PLEASE USE L THIRD NEXT TIME
 
        DO COME FROM '#255~'"?',1SUB#4'$#0"~"#0$#255"''~#1
        PLEASE DO (2) NEXT DO (4) NEXT
        DO USE TWO LS, NOT A SINGLE L
 
        DO COME FROM '#255~'"?',1SUB#5'$#3"~"#0$#255"''~#1
        DO (8) NEXT
        PLEASE END 'HELLO' WITH 'O'
 
        DO COME FROM '#255~'"?',1SUB#6'$#221"~"#0$#255"''~#1
        DO (8) NEXT PLEASE DO (2) NEXT
        DO USE COMMAS TO SEPARATE WORDS
 
        DO COME FROM '#255~'"?',1SUB#7'$#244"~"#0$#255"''~#1
        DO (8) NEXT PLEASE DO (4) NEXT
        PLEASE USE SPACES AFTER PUNCTUATION
 
        DO COME FROM '#255~'"?',1SUB#8'$#55"~"#0$#255"''~#1
        DO (8) NEXT DO (4) NEXT PLEASE DO (2) NEXT
        DO START 'WORLD' WITH A 'W'
 
        DO COME FROM '#255~'"?',1SUB#9'$#248"~"#0$#255"''~#1
        DO (16) NEXT
        PLEASE PLACE AN O IN THE NINTH POSITION
 
        DO COME FROM '#255~'"?',1SUB#10'$#3"~"#0$#255"''~#1
        DO (16) NEXT DO (2) NEXT
        DO USE AN R IN THE MIDDLE OF WORLD
 
        DO COME FROM '#255~'"?',1SUB#11'$#250"~"#0$#255"''~#1
        DO (16) NEXT DO (4) NEXT
        PLEASE LET AN L BE PENULTIMATE
 
        DO COME FROM '#255~'"?',1SUB#12'$#248"~"#0$#255"''~#1
        DO (16) NEXT PLEASE DO (4) NEXT DO (2) NEXT
        DO END WITH A D
 
 (2)    PLEASE RESUME #1
 (4)    DO (2) NEXT DO (2) NEXT DO RESUME #1
 (8)    DO (4) NEXT DO (4) NEXT PLEASE RESUME #1
 (16)   DO (8) NEXT DO (8) NEXT PLEASE RESUME #1"##
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"        PLEASE NOTE THIS PROGRAM RECOGNIZES ""#,
                    output_for_hello_world!(),
                    r##"" USING COME FROM
        DON'T TYPE IN ANYTHING ELSE, OR YOU'LL GET AN ERROR!
 
        PLEASE NOTE: COMPILE WITH ick -m FOR THIS TO WORK.
        
        DO ,1 <- #12
 (1)    DO WRITE IN ,1
        DO GIVE UP
 
        PLEASE NOTE THIS CHECKS EACH CHARACTER IN TURN
        DO COME FROM '#255~'"?',1SUB#1'$#72"~"#0$#255"''~#1
        PLEASE START WITH AN H NEXT TIME
 
        DO COME FROM '#255~'"?',1SUB#2'$#253"~"#0$#255"''~#1
        DO (2) NEXT
        DO REMEMBER THAT E COMES SECOND
 
        DO COME FROM '#255~'"?',1SUB#3'$#7"~"#0$#255"''~#1
        DO (4) NEXT 
        PLEASE USE L THIRD NEXT TIME
 
        DO COME FROM '#255~'"?',1SUB#4'$#0"~"#0$#255"''~#1
        PLEASE DO (2) NEXT DO (4) NEXT
        DO USE TWO LS, NOT A SINGLE L
 
        DO COME FROM '#255~'"?',1SUB#5'$#3"~"#0$#255"''~#1
        DO (8) NEXT
        PLEASE END 'HELLO' WITH 'O'
 
        DO COME FROM '#255~'"?',1SUB#6'$#221"~"#0$#255"''~#1
        DO (8) NEXT PLEASE DO (2) NEXT
        DO USE COMMAS TO SEPARATE WORDS
 
        DO COME FROM '#255~'"?',1SUB#7'$#244"~"#0$#255"''~#1
        DO (8) NEXT PLEASE DO (4) NEXT
        PLEASE USE SPACES AFTER PUNCTUATION
 
        DO COME FROM '#255~'"?',1SUB#8'$#55"~"#0$#255"''~#1
        DO (8) NEXT DO (4) NEXT PLEASE DO (2) NEXT
        DO START 'WORLD' WITH A 'W'
 
        DO COME FROM '#255~'"?',1SUB#9'$#248"~"#0$#255"''~#1
        DO (16) NEXT
        PLEASE PLACE AN O IN THE NINTH POSITION
 
        DO COME FROM '#255~'"?',1SUB#10'$#3"~"#0$#255"''~#1
        DO (16) NEXT DO (2) NEXT
        DO USE AN R IN THE MIDDLE OF WORLD
 
        DO COME FROM '#255~'"?',1SUB#11'$#250"~"#0$#255"''~#1
        DO (16) NEXT DO (4) NEXT
        PLEASE LET AN L BE PENULTIMATE
 
        DO COME FROM '#255~'"?',1SUB#12'$#248"~"#0$#255"''~#1
        DO (16) NEXT PLEASE DO (4) NEXT DO (2) NEXT
        DO END WITH A D
 
 (2)    PLEASE RESUME #1
 (4)    DO (2) NEXT DO (2) NEXT DO RESUME #1
 (8)    DO (4) NEXT DO (4) NEXT PLEASE RESUME #1
 (16)   DO (8) NEXT DO (8) NEXT PLEASE RESUME #1"##
                ),
            )],
        }],
    };
