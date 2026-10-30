//! Hello-world snippet for `ProgrammingLanguageName::Symal`.
//!
//! Imported from the Hello World Collection (`files/SyMAL.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/SyMAL.htm>
//! - <http://helloworldcollection.de/#SyMAL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Symal => "SyMAL",
    versions: [Symal => "SyMAL"],
    extensions: [],
}

const SYMAL: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r" in SyMAL

'",
    output_for_hello_world!(),
    r"'"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Symal,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Symal,
            syntax: SYMAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SYMAL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r" in SyMAL

'",
                            output_for_hello_world!(),
                            r"'"
                        ),
                    )],
                )
            },
        }],
    };
