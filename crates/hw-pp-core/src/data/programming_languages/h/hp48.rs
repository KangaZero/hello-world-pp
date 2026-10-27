//! Hello-world snippet for `ProgrammingLanguageName::Hp48`.
//!
//! Imported from the Hello World Collection (`files/HP-48` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/HP-48>
//! - <http://helloworldcollection.de/#HP-48>
//! - <https://en.wikipedia.org/wiki/RPL_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Concatenative, StackBased, Structured,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Hp48 => "HP-48",
    versions: [Hp48 => "HP-48"],
    extensions: [],
}

const HP48: &str = concat!(
    r"<<
@ ",
    output_for_hello_world!(),
    r#" for the HP-48
@ << and >> are one char each
""#,
    output_for_hello_world!(),
    r#""
>>"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hp48,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Structured, StackBased, Concatenative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hp48,
            syntax: HP48,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HP48,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"<<
@ ",
                            output_for_hello_world!(),
                            r#" for the HP-48
@ << and >> are one char each
""#,
                            output_for_hello_world!(),
                            r#""
>>"#
                        ),
                    )],
                )
            },
        }],
    };
