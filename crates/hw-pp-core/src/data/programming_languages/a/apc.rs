//! Hello-world snippet for `ProgrammingLanguageName::Apc`.
//!
//! Imported from leachim6/hello-world (`a/APC.apc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/APC.apc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Apc => "APC",
    versions: [Apc => "APC"],
    extensions: [Apc => "apc"],
}

const APC: &str = concat!(
    r#"probe program
{
	on_entry log (""#,
    output_for_hello_world!(),
    r#"\n");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Apc,
        extensions: &[FileExtension::Apc],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Apc,
            syntax: APC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    APC,
                    [(
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
                )
            },
        }],
    };
