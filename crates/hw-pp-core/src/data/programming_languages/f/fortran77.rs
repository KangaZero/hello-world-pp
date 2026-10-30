//! Hello-world snippet for `ProgrammingLanguageName::Fortran77`.
//!
//! Imported from leachim6/hello-world (`f/Fortran77.f77` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Fortran77.f77>
//! - <https://en.wikipedia.org/wiki/Fortran> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Array, Generic, Imperative, ObjectOriented, Procedural, Structured,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Fortran77 => "Fortran77",
    versions: [Fortran77 => "Fortran77"],
    extensions: [F77 => "f77"],
}

const FORTRAN77: &str = concat!(
    r"      PROGRAM HELLOW
      WRITE(UNIT=*, FMT=*) '",
    output_for_hello_world!(),
    r"'
      END"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fortran77,
        extensions: &[FileExtension::F77],
        categories: &[Compiled, Historical],
        families: &[],
        paradigms: &[
            Imperative,
            Procedural,
            Structured,
            ObjectOriented,
            Array,
            Generic,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fortran77,
            syntax: FORTRAN77,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FORTRAN77,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"      PROGRAM HELLOW
      WRITE(UNIT=*, FMT=*) '",
                            output_for_hello_world!(),
                            r"'
      END"
                        ),
                    )],
                )
            },
        }],
    };
