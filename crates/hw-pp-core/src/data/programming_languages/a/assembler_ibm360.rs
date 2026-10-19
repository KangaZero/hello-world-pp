//! Hello-world snippet for `ProgrammingLanguageName::AssemblerIbm360`.
//!
//! Imported from leachim6/hello-world (`a/Assembler IBM360.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerIbm360 => "Assembler IBM360",
    versions: [AssemblerIbm360 => "Assembler IBM360"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerIbm360,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerIbm360,
            syntax: concat!(
                r"         LA    1,MSGAREA     Point Register 1 to message area
         SVC   35            Invoke SVC 35 (Write to Operator) 
         BR    14            Return
MSGAREA  EQU   *             Message Area
         DC    AL2(19)       Total area length = 19 (Prefix length:4 + Data Length:15) 
         DC    XL2'00'       2 bytes binary of zeros
         DC    C'",
                output_for_hello_world!(),
                r"'  Text to be written to system console
         END"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"         LA    1,MSGAREA     Point Register 1 to message area
         SVC   35            Invoke SVC 35 (Write to Operator) 
         BR    14            Return
MSGAREA  EQU   *             Message Area
         DC    AL2(19)       Total area length = 19 (Prefix length:4 + Data Length:15) 
         DC    XL2'00'       2 bytes binary of zeros
         DC    C'",
                    output_for_hello_world!(),
                    r"'  Text to be written to system console
         END"
                ),
            )],
        }],
    };
