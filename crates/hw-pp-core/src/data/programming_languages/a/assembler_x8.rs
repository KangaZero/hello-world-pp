//! Hello-world snippet for `ProgrammingLanguageName::AssemblerX8`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(X8).htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28X8%29.htm>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(X8)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerX8 => "Assembler (X8)",
    versions: [AssemblerX8 => "Assembler (X8)"],
    extensions: [],
}

const ASSEMBLER_X8: &str = concat!(
    r#"" "#,
    output_for_hello_world!(),
    r#" in Assembler for the Electrologica X8 (ca. 1965)

'BEGIN' TEL='1 000 000',  TTY=8
M[24]:
        GOTO(:START)
M[(64+TTY*4)]:
TAR:
M[(64+38*4)]:
TPAR:
        
M['400']:
        
START:
        A=:HELLO DESCR                  " point to I/O descriptor
        TAR[0]=A                        " set address of string
        TAR[1]=-A                       " set IFT = -1
        A=D18
        TAR[2]=A
        AFON(TTY)                       " send the message
LOOP:   GOTO(:LOOP)                     " spin (since there is no halt)
        
D18:    '001 000 000'

HELLO DESCR:
        0
        (17*TEL + :DAG[-1])
        0

DAG:    37                              " letters, red print
        5                               " H
        16                              " E
        9                               " L
        9                               " L
        3                               " O
        27                              " figures
        6                               " comma
        31                              " letters
        4                               " space
        25                              " W
        3                               " O
        10                              " R
        9                               " L
        18                              " D
        2                               " cr
        8                               " nl"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerX8,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerX8,
            syntax: ASSEMBLER_X8,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_X8,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"" "#,
                            output_for_hello_world!(),
                            r#" in Assembler for the Electrologica X8 (ca. 1965)

'BEGIN' TEL='1 000 000',  TTY=8
M[24]:
        GOTO(:START)
M[(64+TTY*4)]:
TAR:
M[(64+38*4)]:
TPAR:
        
M['400']:
        
START:
        A=:HELLO DESCR                  " point to I/O descriptor
        TAR[0]=A                        " set address of string
        TAR[1]=-A                       " set IFT = -1
        A=D18
        TAR[2]=A
        AFON(TTY)                       " send the message
LOOP:   GOTO(:LOOP)                     " spin (since there is no halt)
        
D18:    '001 000 000'

HELLO DESCR:
        0
        (17*TEL + :DAG[-1])
        0

DAG:    37                              " letters, red print
        5                               " H
        16                              " E
        9                               " L
        9                               " L
        3                               " O
        27                              " figures
        6                               " comma
        31                              " letters
        4                               " space
        25                              " W
        3                               " O
        10                              " R
        9                               " L
        18                              " D
        2                               " cr
        8                               " nl"#
                        ),
                    )],
                )
            },
        }],
    };
