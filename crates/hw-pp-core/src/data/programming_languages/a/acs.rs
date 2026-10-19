//! Hello-world snippet for `ProgrammingLanguageName::Acs`.
//!
//! Imported from leachim6/hello-world (`a/ACS.acs` at `a152253`),
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
    language: Acs => "ACS",
    versions: [Acs => "ACS"],
    extensions: [Acs => "acs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Acs,
        extensions: &[FileExtension::Acs],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Acs,
            syntax: concat!(
                r#"#include "zcommon.acs"

Script 1 OPEN
{
  Print(s: ""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#include "zcommon.acs"

Script 1 OPEN
{
  Print(s: ""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
            )],
        }],
    };
