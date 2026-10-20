//! Hello-world snippet for `ProgrammingLanguageName::Ttcn3`.
//!
//! Imported from leachim6/hello-world (`t/TTCN3.ttcn` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/TTCN3.ttcn>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ttcn3 => "TTCN3",
    versions: [Ttcn3 => "TTCN3"],
    extensions: [Ttcn => "ttcn"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ttcn3,
        extensions: &[FileExtension::Ttcn],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ttcn3,
            syntax: concat!(
                r#"module HelloWorld
{
    control
    {
        log(""#,
                output_for_hello_world!(),
                r#"");
    }
 }"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"module HelloWorld
{
    control
    {
        log(""#,
                    output_for_hello_world!(),
                    r#"");
    }
 }"#
                ),
            )],
        }],
    };
