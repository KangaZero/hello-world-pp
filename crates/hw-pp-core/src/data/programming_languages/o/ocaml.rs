//! Hello-world snippet for `ProgrammingLanguageName::OCaml`.
//!
//! Sources:
//! - <https://ocaml.org/docs/tour-of-ocaml>
//! - <https://ocaml.org/docs/your-first-program>
//! - <https://www.wikidata.org/wiki/Q212587> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{Ml, OCaml},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: OCaml => "OCaml",
    versions: [OCaml5 => "OCaml 5"],
    extensions: [Ml => "ml", Mli => "mli"],
}

const O_CAML5: &str = concat!(
    r#"let () = print_endline ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OCaml,
        extensions: &[FileExtension::Ml, FileExtension::Mli],
        categories: &[Compiled],
        families: &[Ml, OCaml],
        paradigms: &[Imperative, ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OCaml5,
            syntax: O_CAML5,
            tokenized_syntax: &const {
                checked_token_lengths(
                    O_CAML5,
                    [
                        (TokenKind::Keyword, "let"),
                        (TokenKind::Plain, " () "),
                        (TokenKind::Keyword, "="),
                        (TokenKind::Plain, " print_endline "),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                    ],
                )
            },
        }],
    };
