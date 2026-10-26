//! Hello-world snippet for `ProgrammingLanguageName::Strelnokoff`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Strelnokoff>
//! - <https://esolangs.org/wiki/Strelnokoff>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Strelnokoff => "Strelnokoff",
    versions: [Strelnokoff => "Strelnokoff"],
    extensions: [],
}

const STRELNOKOFF: &str = concat!(
    r"REM ",
    output_for_hello_world!(),
    r" IN STRELNOKOFF
REM CHRIS PRESSEY MARCH 24 2001
X = (X / X) * X + (X = 0) * (T =  0) * (PRINT CHAR 'H' - 'H' +  1)
X = (X / X) * X + (X = 0) * (T =  1) * (PRINT CHAR 'e' - 'e' +  2)
X = (X / X) * X + (X = 0) * (T =  2) * (PRINT CHAR 'l' - 'l' +  3)
X = (X / X) * X + (X = 0) * (T =  3) * (PRINT CHAR 'l' - 'l' +  4)
X = (X / X) * X + (X = 0) * (T =  4) * (PRINT CHAR 'o' - 'o' +  5)
X = (X / X) * X + (X = 0) * (T =  5) * (PRINT CHAR ',' - ',' +  6)
X = (X / X) * X + (X = 0) * (T =  6) * (PRINT CHAR ' ' - ' ' +  7)
X = (X / X) * X + (X = 0) * (T =  7) * (PRINT CHAR 'w' - 'w' +  8)
X = (X / X) * X + (X = 0) * (T =  8) * (PRINT CHAR 'o' - 'o' +  9)
X = (X / X) * X + (X = 0) * (T =  9) * (PRINT CHAR 'r' - 'r' + 10)
X = (X / X) * X + (X = 0) * (T = 10) * (PRINT CHAR 'l' - 'l' + 11)
X = (X / X) * X + (X = 0) * (T = 11) * (PRINT CHAR 'd' - 'd' + 12)
X = (X / X) * X + (X = 0) * (T = 12) * (PRINT CHAR '!' - '!' + 13)
X = (T = X) * 0 + (X > T) * X REM RESET FLAG
T = (X / X) * X + (X = 0) * T REM INCREMENT TICK"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Strelnokoff,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Strelnokoff,
            syntax: STRELNOKOFF,
            tokenized_syntax: &const {
                checked_token_lengths(
                    STRELNOKOFF,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"REM ",
                            output_for_hello_world!(),
                            r" IN STRELNOKOFF
REM CHRIS PRESSEY MARCH 24 2001
X = (X / X) * X + (X = 0) * (T =  0) * (PRINT CHAR 'H' - 'H' +  1)
X = (X / X) * X + (X = 0) * (T =  1) * (PRINT CHAR 'e' - 'e' +  2)
X = (X / X) * X + (X = 0) * (T =  2) * (PRINT CHAR 'l' - 'l' +  3)
X = (X / X) * X + (X = 0) * (T =  3) * (PRINT CHAR 'l' - 'l' +  4)
X = (X / X) * X + (X = 0) * (T =  4) * (PRINT CHAR 'o' - 'o' +  5)
X = (X / X) * X + (X = 0) * (T =  5) * (PRINT CHAR ',' - ',' +  6)
X = (X / X) * X + (X = 0) * (T =  6) * (PRINT CHAR ' ' - ' ' +  7)
X = (X / X) * X + (X = 0) * (T =  7) * (PRINT CHAR 'w' - 'w' +  8)
X = (X / X) * X + (X = 0) * (T =  8) * (PRINT CHAR 'o' - 'o' +  9)
X = (X / X) * X + (X = 0) * (T =  9) * (PRINT CHAR 'r' - 'r' + 10)
X = (X / X) * X + (X = 0) * (T = 10) * (PRINT CHAR 'l' - 'l' + 11)
X = (X / X) * X + (X = 0) * (T = 11) * (PRINT CHAR 'd' - 'd' + 12)
X = (X / X) * X + (X = 0) * (T = 12) * (PRINT CHAR '!' - '!' + 13)
X = (T = X) * 0 + (X > T) * X REM RESET FLAG
T = (X / X) * X + (X = 0) * T REM INCREMENT TICK"
                        ),
                    )],
                )
            },
        }],
    };
