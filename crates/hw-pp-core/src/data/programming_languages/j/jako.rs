//! Hello-world snippet for `ProgrammingLanguageName::Jako`.
//!
//! Imported from the Hello World Collection (`files/Jako.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Jako.txt>
//! - <http://helloworldcollection.de/#Jako>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Jako => "Jako",
    versions: [Jako => "Jako"],
    extensions: [],
}

const JAKO: &str = concat!(
    r"# ",
    output_for_hello_world!(),
    r#" in Jako

use sys;

sys::print(""#,
    output_for_hello_world!(),
    r#"\n");"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jako,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jako,
            syntax: JAKO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JAKO,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"# ",
                            output_for_hello_world!(),
                            r#" in Jako

use sys;

sys::print(""#,
                            output_for_hello_world!(),
                            r#"\n");"#
                        ),
                    )],
                )
            },
        }],
    };
