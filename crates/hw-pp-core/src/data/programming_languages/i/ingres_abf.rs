//! Hello-world snippet for `ProgrammingLanguageName::IngresAbf`.
//!
//! Imported from the Hello World Collection (`files/Ingres&nbsp;ABF.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Ingres%26nbsp%3BABF.txt>
//! - <http://helloworldcollection.de/#Ingres%C2%A0ABF>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: IngresAbf => "Ingres ABF",
    versions: [IngresAbf => "Ingres ABF"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::IngresAbf,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::IngresAbf,
            syntax: concat!(
                r"/* ",
                output_for_hello_world!(),
                r" in Ingres ABF */
procedure hello =
begin
  message '",
                output_for_hello_world!(),
                r"' with style=popup;
end"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"/* ",
                    output_for_hello_world!(),
                    r" in Ingres ABF */
procedure hello =
begin
  message '",
                    output_for_hello_world!(),
                    r"' with style=popup;
end"
                ),
            )],
        }],
    };
