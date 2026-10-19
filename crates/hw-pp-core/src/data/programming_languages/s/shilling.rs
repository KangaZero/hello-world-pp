//! Hello-world snippet for `ProgrammingLanguageName::Shilling`.
//!
//! Imported from leachim6/hello-world (PR #1603 `s/Shilling.shil`),
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
    language: Shilling => "Shilling",
    versions: [Shilling => "Shilling"],
    extensions: [Shil => "shil"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Shilling,
        extensions: &[FileExtension::Shil],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Shilling,
            syntax: concat!(
                r#"start HelloWorld;

blue HelloWorld {
    parrot(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"start HelloWorld;

blue HelloWorld {
    parrot(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
            )],
        }],
    };
