//! Hello-world snippet for `ProgrammingLanguageName::Fortran77`.
//!
//! Imported from leachim6/hello-world (`f/Fortran77.f77` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Fortran77.f77>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Fortran77 => "Fortran77",
    versions: [Fortran77 => "Fortran77"],
    extensions: [F77 => "f77"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fortran77,
        extensions: &[FileExtension::F77],
        categories: &[Compiled, Historical],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fortran77,
            syntax: concat!(
                r"      PROGRAM HELLOW
      WRITE(UNIT=*, FMT=*) '",
                output_for_hello_world!(),
                r"'
      END"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"      PROGRAM HELLOW
      WRITE(UNIT=*, FMT=*) '",
                    output_for_hello_world!(),
                    r"'
      END"
                ),
            )],
        }],
    };
