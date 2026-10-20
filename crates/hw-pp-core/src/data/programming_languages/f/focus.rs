//! Hello-world snippet for `ProgrammingLanguageName::Focus`.
//!
//! Imported from the Hello World Collection (`files/FOCUS.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/FOCUS.txt>
//! - <http://helloworldcollection.de/#FOCUS>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Focus => "FOCUS",
    versions: [Focus => "FOCUS"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Focus,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Focus,
            syntax: concat!(
                r"-* ",
                output_for_hello_world!(),
                r" in FOCUS

-TYPE ",
                output_for_hello_world!()
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"-* ",
                    output_for_hello_world!(),
                    r" in FOCUS

-TYPE ",
                    output_for_hello_world!()
                ),
            )],
        }],
    };
