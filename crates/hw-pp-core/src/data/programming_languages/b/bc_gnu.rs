//! Hello-world snippet for `ProgrammingLanguageName::BcGnu`.
//!
//! Imported from leachim6/hello-world (`b/BC GNU.bc` at `a152253`),
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
    language: BcGnu => "BC GNU",
    versions: [BcGnu => "BC GNU"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BcGnu,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BcGnu,
            syntax: concat!(
                r#"print ""#,
                output_for_hello_world!(),
                r#""
quit"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"print ""#,
                    output_for_hello_world!(),
                    r#""
quit"#
                ),
            )],
        }],
    };
