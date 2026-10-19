//! Hello-world snippets for `ProgrammingLanguageName::OCaml`.
//!
//! Sources:
//! - <https://ocaml.org/docs/tour-of-ocaml>
//! - <https://ocaml.org/docs/your-first-program>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: OCaml => "OCaml",
    versions: [OCaml5 => "OCaml 5"],
    extensions: [Ml => "ml", Mli => "mli"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OCaml,
        extensions: &[FileExtension::Ml, FileExtension::Mli],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OCaml5,
            syntax: concat!(
                r#"let () = print_endline ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "let"),
                (TokenKind::Plain, " () "),
                (TokenKind::Keyword, "="),
                (TokenKind::Plain, " print_endline "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    };
