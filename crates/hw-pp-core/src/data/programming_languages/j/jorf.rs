//! Hello-world snippet for `ProgrammingLanguageName::Jorf`.
//!
//! Imported from the Hello World Collection (`files/Jorf.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Jorf.htm>
//! - <http://helloworldcollection.de/#Jorf>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Jorf => "Jorf",
    versions: [Jorf => "Jorf"],
    extensions: [],
}

const JORF: &str = concat!(
    r"| ",
    output_for_hello_world!(),
    r#" in Jorf

Hello:Start
  Msg:Add("Hello", "Ok")
    "#,
    output_for_hello_world!(),
    r"
  Return"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jorf,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jorf,
            syntax: JORF,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JORF,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"| ",
                            output_for_hello_world!(),
                            r#" in Jorf

Hello:Start
  Msg:Add("Hello", "Ok")
    "#,
                            output_for_hello_world!(),
                            r"
  Return"
                        ),
                    )],
                )
            },
        }],
    };
