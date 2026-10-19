//! Hello-world snippet for `ProgrammingLanguageName::Apc`.
//!
//! Imported from leachim6/hello-world (`a/APC.apc` at `a152253`),
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
    language: Apc => "APC",
    versions: [Apc => "APC"],
    extensions: [Apc => "apc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Apc,
        extensions: &[FileExtension::Apc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Apc,
            syntax: concat!(
                r#"probe program
{
	on_entry log (""#,
                output_for_hello_world!(),
                r#"\n");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"probe program
{
	on_entry log (""#,
                    output_for_hello_world!(),
                    r#"\n");
}"#
                ),
            )],
        }],
    };
