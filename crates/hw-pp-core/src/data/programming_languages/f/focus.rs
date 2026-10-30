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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Focus => "FOCUS",
    versions: [Focus => "FOCUS"],
    extensions: [],
}

const FOCUS: &str = concat!(
    r"-* ",
    output_for_hello_world!(),
    r" in FOCUS

-TYPE ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Focus,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Focus,
            syntax: FOCUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FOCUS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"-* ",
                            output_for_hello_world!(),
                            r" in FOCUS

-TYPE ",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };
