//! Hello-world snippet for `ProgrammingLanguageName::Stylus`.
//!
//! Imported from leachim6/hello-world (`s/Stylus.styl` at `a152253`),
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
    language: Stylus => "Stylus",
    versions: [Stylus => "Stylus"],
    extensions: [Styl => "styl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Stylus,
        extensions: &[FileExtension::Styl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Stylus,
            syntax: concat!(
                r#"body::before
	content: ""#,
                output_for_hello_world!(),
                r#"" "#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"body::before
	content: ""#,
                    output_for_hello_world!(),
                    r#"" "#
                ),
            )],
        }],
    };
