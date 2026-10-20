//! Hello-world snippet for `ProgrammingLanguageName::Powerscript`.
//!
//! Imported from the Hello World Collection (`files/PowerScript.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/PowerScript.txt>
//! - <http://helloworldcollection.de/#PowerScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Powerscript => "PowerScript",
    versions: [Powerscript => "PowerScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Powerscript,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Powerscript,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r#" in PowerScript

MessageBox("",""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in PowerScript

MessageBox("",""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
