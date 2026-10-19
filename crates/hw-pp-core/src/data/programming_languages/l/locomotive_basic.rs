//! Hello-world snippet for `ProgrammingLanguageName::LocomotiveBasic`.
//!
//! Imported from leachim6/hello-world (`l/Locomotive Basic.b` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: LocomotiveBasic => "Locomotive Basic",
    versions: [LocomotiveBasic => "Locomotive Basic"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LocomotiveBasic,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LocomotiveBasic,
            syntax: concat!(
                r#"10 print ""#,
                output_for_hello_world!(),
                r#""
run"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"10 print ""#,
                    output_for_hello_world!(),
                    r#""
run"#
                ),
            )],
        }],
    };
