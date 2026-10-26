//! Hello-world snippet for `ProgrammingLanguageName::Wenyan`.
//!
//! Imported from the Hello World Collection (`files/wenyan.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/wenyan.htm>
//! - <http://helloworldcollection.de/#wenyan>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Wenyan => "wenyan",
    versions: [Wenyan => "wenyan"],
    extensions: [],
}

const WENYAN: &str = concat!(
    r"注曰。「「 ",
    output_for_hello_world!(),
    r" in wenyan 」」。
吾有一言。曰「「問天地好在。」」。書之。"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wenyan,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wenyan,
            syntax: WENYAN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WENYAN,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"注曰。「「 ",
                            output_for_hello_world!(),
                            r" in wenyan 」」。
吾有一言。曰「「問天地好在。」」。書之。"
                        ),
                    )],
                )
            },
        }],
    };
