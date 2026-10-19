//! Hello-world snippet for `ProgrammingLanguageName::CamlLight`.
//!
//! Imported from the Hello World Collection (`files/CAML-Light` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/CAML-Light>
//! - <http://helloworldcollection.de/#CAML-Light>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: CamlLight => "CAML-Light",
    versions: [CamlLight => "CAML-Light"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CamlLight,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CamlLight,
            syntax: concat!(
                r"(* ",
                output_for_hello_world!(),
                r#" in CAML Light *)

let hello =
   print_string ""#,
                output_for_hello_world!(),
                r#"";
;;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"(* ",
                    output_for_hello_world!(),
                    r#" in CAML Light *)

let hello =
   print_string ""#,
                    output_for_hello_world!(),
                    r#"";
;;"#
                ),
            )],
        }],
    };
