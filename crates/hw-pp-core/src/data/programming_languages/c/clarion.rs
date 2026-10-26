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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Clarion => "Clarion",
    versions: [Clarion => "Clarion"],
    extensions: [],
}

const CLARION: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clarion,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clarion,
            syntax: CLARION,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CLARION,
                    [(
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
                )
            },
        }],
    };
