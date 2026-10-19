//! Hello-world snippet for `ProgrammingLanguageName::Clarion`.
//!
//! Imported from the Hello World Collection (`files/Clarion.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Clarion.txt>
//! - <http://helloworldcollection.de/#Clarion>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Clarion => "Clarion",
    versions: [Clarion => "Clarion"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clarion,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clarion,
            syntax: concat!(
                r"!",
                output_for_hello_world!(),
                r" in Clarion 
 
  PROGRAM
 
 MAP
 END
 
 CODE
 
 MESSAGE('",
                output_for_hello_world!(),
                r"')
 
 RETURN"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"!",
                    output_for_hello_world!(),
                    r" in Clarion 
 
  PROGRAM
 
 MAP
 END
 
 CODE
 
 MESSAGE('",
                    output_for_hello_world!(),
                    r"')
 
 RETURN"
                ),
            )],
        }],
    };
