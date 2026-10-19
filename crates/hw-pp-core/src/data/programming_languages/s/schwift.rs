//! Hello-world snippet for `ProgrammingLanguageName::Schwift`.
//!
//! Imported from leachim6/hello-world (`s/SCHWift.y` at `a152253`),
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
    language: Schwift => "SCHWift",
    versions: [Schwift => "SCHWift"],
    extensions: [Y => "y"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Schwift,
        extensions: &[FileExtension::Y],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Schwift,
            syntax: concat!(
                r#"show me what you got ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"show me what you got ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
