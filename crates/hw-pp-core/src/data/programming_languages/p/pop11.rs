//! Hello-world snippet for `ProgrammingLanguageName::Pop11`.
//!
//! Imported from the Hello World Collection (`files/POP-11.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/POP-11.txt>
//! - <http://helloworldcollection.de/#POP-11>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Lisp,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Pop11 => "POP-11",
    versions: [Pop11 => "POP-11"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pop11,
        extensions: &[],
        categories: &[],
        families: &[Lisp],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pop11,
            syntax: concat!(
                r";;; ",
                output_for_hello_world!(),
                r#" in POP-11

: vars outputtext;
: ""#,
                output_for_hello_world!(),
                r#"" -> outputtext;
: outputtext =>

** "#,
                output_for_hello_world!()
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r";;; ",
                    output_for_hello_world!(),
                    r#" in POP-11

: vars outputtext;
: ""#,
                    output_for_hello_world!(),
                    r#"" -> outputtext;
: outputtext =>

** "#,
                    output_for_hello_world!()
                ),
            )],
        }],
    };
