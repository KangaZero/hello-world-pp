//! Hello-world snippet for `ProgrammingLanguageName::N4gl`.
//!
//! Imported from leachim6/hello-world (`#/4gl.4gl` at `a152253`),
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
    language: N4gl => "4gl",
    versions: [N4gl => "4gl"],
    extensions: [E4gl => "4gl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N4gl,
        extensions: &[FileExtension::E4gl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N4gl,
            syntax: concat!(
                r#"main
    display ""#,
                output_for_hello_world!(),
                r#""
end main"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"main
    display ""#,
                    output_for_hello_world!(),
                    r#""
end main"#
                ),
            )],
        }],
    };
