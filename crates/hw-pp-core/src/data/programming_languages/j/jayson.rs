//! Hello-world snippet for `ProgrammingLanguageName::Jayson`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#JAYSON>
//! - <https://esolangs.org/wiki/JAYSON>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Jayson => "JAYSON",
    versions: [Jayson => "JAYSON"],
    extensions: [],
}

const JAYSON: &str = concat!(
    r#"{
  "program": {
    "main": [
      {
        "operation": "print",
        "value": ""#,
    output_for_hello_world!(),
    r#""
      }
    ]
  }
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jayson,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jayson,
            syntax: JAYSON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JAYSON,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"{
  "program": {
    "main": [
      {
        "operation": "print",
        "value": ""#,
                            output_for_hello_world!(),
                            r#""
      }
    ]
  }
}"#
                        ),
                    )],
                )
            },
        }],
    };
