//! Hello-world snippet for `ProgrammingLanguageName::Fortress`.
//!
//! Imported from the Hello World Collection (`files/Fortress.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Fortress.txt>
//! - <http://helloworldcollection.de/#Fortress>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Fortran,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Fortress => "Fortress",
    versions: [Fortress => "Fortress"],
    extensions: [],
}

const FORTRESS: &str = concat!(
    r"(* ",
    output_for_hello_world!(),
    r#" in Fortress *)

export Executable
run(args) = print ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fortress,
        extensions: &[],
        categories: &[],
        families: &[Fortran],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fortress,
            syntax: FORTRESS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FORTRESS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"(* ",
                            output_for_hello_world!(),
                            r#" in Fortress *)

export Executable
run(args) = print ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
