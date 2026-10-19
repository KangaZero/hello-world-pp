//! Hello-world snippet for `ProgrammingLanguageName::AssemblerTms9900Ti994a`.
//!
//! Imported from leachim6/hello-world (`a/Assembler tms9900 ti99 4a.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20tms9900%20ti99%204a.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerTms9900Ti994a => "Assembler tms9900 ti99 4a",
    versions: [AssemblerTms9900Ti994a => "Assembler tms9900 ti99 4a"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerTms9900Ti994a,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerTms9900Ti994a,
            syntax: concat!(
                r"
* ",
                output_for_hello_world!(),
                r" for TI-99/4a computer with TMS9900 CPU
* by Frog ( http://frog.enlight.ru )
* 15 April 2016
*
****************************************************************************

        DEF     START

WRKSP   EQU     >8300

VDPWD   EQU     >8C00        
VDPWA   EQU     >8C02


START   LIMI    0            
        LWPI    WRKSP

        LI      R0,>0000
        ORI     R0,>4000
        SWPB    R0
        MOVB    R0,@VDPWA
        SWPB    R0
        MOVB    R0,@VDPWA

        LI      R1,HELLOWORLD   
        LI      R2,12          

NEXTCHAR
        MOVB    *R1+,@VDPWD     
        DEC     R2
        JNE     NEXTCHAR

LOOPBACK
        JMP LOOPBACK


HELLOWORLD
        TEXT '",
                output_for_hello_world!(),
                r"'

        BYTE 0

        END"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"
* ",
                    output_for_hello_world!(),
                    r" for TI-99/4a computer with TMS9900 CPU
* by Frog ( http://frog.enlight.ru )
* 15 April 2016
*
****************************************************************************

        DEF     START

WRKSP   EQU     >8300

VDPWD   EQU     >8C00        
VDPWA   EQU     >8C02


START   LIMI    0            
        LWPI    WRKSP

        LI      R0,>0000
        ORI     R0,>4000
        SWPB    R0
        MOVB    R0,@VDPWA
        SWPB    R0
        MOVB    R0,@VDPWA

        LI      R1,HELLOWORLD   
        LI      R2,12          

NEXTCHAR
        MOVB    *R1+,@VDPWD     
        DEC     R2
        JNE     NEXTCHAR

LOOPBACK
        JMP LOOPBACK


HELLOWORLD
        TEXT '",
                    output_for_hello_world!(),
                    r"'

        BYTE 0

        END"
                ),
            )],
        }],
    };
