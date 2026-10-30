//! Hello-world snippet for `ProgrammingLanguageName::Marmelade`.
//!
//! Imported from the Hello World Collection (`files/Marmelade.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Marmelade.htm>
//! - <http://helloworldcollection.de/#Marmelade>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Marmelade => "Marmelade",
    versions: [Marmelade => "Marmelade"],
    extensions: [],
}

const MARMELADE: &str = concat!(
    r"/ ",
    output_for_hello_world!(),
    r#" in Marmelade

sayln ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Marmelade,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Marmelade,
            syntax: MARMELADE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MARMELADE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"/ ",
                            output_for_hello_world!(),
                            r#" in Marmelade

sayln ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
