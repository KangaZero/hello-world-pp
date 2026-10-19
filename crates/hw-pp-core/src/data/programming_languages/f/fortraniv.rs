//! Hello-world snippet for `ProgrammingLanguageName::Fortraniv`.
//!
//! Imported from the Hello World Collection (`files/FortranIV` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/FortranIV>
//! - <http://helloworldcollection.de/#FortranIV>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Fortraniv => "FortranIV",
    versions: [Fortraniv => "FortranIV"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fortraniv,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fortraniv,
            syntax: concat!(
                r"       PROGRAM HELLO
c
C      ",
                output_for_hello_world!(),
                r" in Fortran IV (supposedly for a TR440)
c
       WRITE (6,'('' ",
                output_for_hello_world!(),
                r"'')')
       END"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"       PROGRAM HELLO
c
C      ",
                    output_for_hello_world!(),
                    r" in Fortran IV (supposedly for a TR440)
c
       WRITE (6,'('' ",
                    output_for_hello_world!(),
                    r"'')')
       END"
                ),
            )],
        }],
    };
