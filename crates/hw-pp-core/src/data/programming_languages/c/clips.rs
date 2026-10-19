//! Hello-world snippet for `ProgrammingLanguageName::Clips`.
//!
//! Imported from leachim6/hello-world (`c/CLIPS.clips` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Clips => "CLIPS",
    versions: [Clips => "CLIPS"],
    extensions: [Clips => "clips"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clips,
        extensions: &[FileExtension::Clips],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clips,
            syntax: concat!(
                r#"(defrule hw
    (f ?x)
=>
    (printout t ?x crlf))

(assert (f ""#,
                output_for_hello_world!(),
                r#""))

(run)"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"(defrule hw
    (f ?x)
=>
    (printout t ?x crlf))

(assert (f ""#,
                    output_for_hello_world!(),
                    r#""))

(run)"#
                ),
            )],
        }],
    };
