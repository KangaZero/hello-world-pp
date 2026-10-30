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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Mana => "Mana",
    versions: [Mana => "Mana"],
    extensions: [Mn => "mn"],
}

const MANA: &str = concat!(
    r#"action HelloWorldActor::init
{
 print (""#,
    output_for_hello_world!(),
    r#"\n") ;
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mana,
        extensions: &[FileExtension::Mn],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mana,
            syntax: MANA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MANA,
                    [(
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
                )
            },
        }],
    };
