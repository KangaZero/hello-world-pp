//! Hello-world snippet for `ProgrammingLanguageName::Snobol`.
//!
//! Imported from leachim6/hello-world (`s/SNOBOL` at `a152253`),
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
    language: Snobol => "SNOBOL",
    versions: [Snobol => "SNOBOL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Snobol,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Snobol,
            syntax: concat!(
                r#"          OUTPUT = ""#,
                output_for_hello_world!(),
                r#""
END"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"          OUTPUT = ""#,
                    output_for_hello_world!(),
                    r#""
END"#
                ),
            )],
        }],
    };
