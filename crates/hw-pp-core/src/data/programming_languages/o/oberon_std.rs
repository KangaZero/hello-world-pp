//! Hello-world snippet for `ProgrammingLanguageName::OberonStd`.
//!
//! Imported from the Hello World Collection (`files/Oberon.std` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Oberon.std>
//! - <http://helloworldcollection.de/#Oberon.std>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: OberonStd => "Oberon.std",
    versions: [OberonStd => "Oberon.std"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OberonStd,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OberonStd,
            syntax: concat!(
                r"(* ",
                output_for_hello_world!(),
                r#" in Oberon for standard operating systems *)

MODULE HelloWorld;
IMPORT Out;
BEGIN
  Out.String(""#,
                output_for_hello_world!(),
                r#"");
  Out.Ln;
END HelloWorld;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"(* ",
                    output_for_hello_world!(),
                    r#" in Oberon for standard operating systems *)

MODULE HelloWorld;
IMPORT Out;
BEGIN
  Out.String(""#,
                    output_for_hello_world!(),
                    r#"");
  Out.Ln;
END HelloWorld;"#
                ),
            )],
        }],
    };
