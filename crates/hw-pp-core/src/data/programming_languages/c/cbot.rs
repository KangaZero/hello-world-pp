//! Hello-world snippet for `ProgrammingLanguageName::Cbot`.
//!
//! Imported from leachim6/hello-world (`c/Cbot.txt` at `a152253`),
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
    language: Cbot => "Cbot",
    versions: [Cbot => "Cbot"],
    extensions: [Txt => "txt"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cbot,
        extensions: &[FileExtension::Txt],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cbot,
            syntax: concat!(
                r#"extern void object::New()
{

message(""#,
                output_for_hello_world!(),
                r#"");

}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"extern void object::New()
{

message(""#,
                    output_for_hello_world!(),
                    r#"");

}"#
                ),
            )],
        }],
    };
