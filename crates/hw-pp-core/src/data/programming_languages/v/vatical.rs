//! Hello-world snippet for `ProgrammingLanguageName::Vatical`.
//!
//! Imported from the Hello World Collection (`files/Vatical.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Vatical.htm>
//! - <http://helloworldcollection.de/#Vatical>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Vatical => "Vatical",
    versions: [Vatical => "Vatical"],
    extensions: [],
}

const VATICAL: &str = concat!(
    r"+ ",
    output_for_hello_world!(),
    r#" in Vatical

LITURGY:
	PRAY ""#,
    output_for_hello_world!(),
    r#""
AMEN."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vatical,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vatical,
            syntax: VATICAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VATICAL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"+ ",
                            output_for_hello_world!(),
                            r#" in Vatical

LITURGY:
	PRAY ""#,
                            output_for_hello_world!(),
                            r#""
AMEN."#
                        ),
                    )],
                )
            },
        }],
    };
