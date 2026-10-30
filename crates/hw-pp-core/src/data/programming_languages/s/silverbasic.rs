//! Hello-world snippet for `ProgrammingLanguageName::Silverbasic`.
//!
//! Imported from the Hello World Collection (`files/SilverBasic.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/SilverBasic.txt>
//! - <http://helloworldcollection.de/#SilverBasic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Silverbasic => "SilverBasic",
    versions: [Silverbasic => "SilverBasic"],
    extensions: [],
}

const SILVERBASIC: &str = concat!(
    r"//",
    output_for_hello_world!(),
    r#" in SilverBasic

PRINT ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Silverbasic,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Silverbasic,
            syntax: SILVERBASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SILVERBASIC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"//",
                            output_for_hello_world!(),
                            r#" in SilverBasic

PRINT ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
