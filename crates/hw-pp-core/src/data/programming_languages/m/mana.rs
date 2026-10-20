//! Hello-world snippet for `ProgrammingLanguageName::Mana`.
//!
//! Imported from leachim6/hello-world (`m/Mana.mn` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mana.mn>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Mana => "Mana",
    versions: [Mana => "Mana"],
    extensions: [Mn => "mn"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mana,
        extensions: &[FileExtension::Mn],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mana,
            syntax: concat!(
                r#"action HelloWorldActor::init
{
 print (""#,
                output_for_hello_world!(),
                r#"\n") ;
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"action HelloWorldActor::init
{
 print (""#,
                    output_for_hello_world!(),
                    r#"\n") ;
}"#
                ),
            )],
        }],
    };
