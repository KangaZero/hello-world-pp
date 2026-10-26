//! Hello-world snippet for `ProgrammingLanguageName::Lasso`.
//!
//! Imported from the Hello World Collection (`files/Lasso.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Lasso.htm>
//! - <http://helloworldcollection.de/#Lasso>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Lasso => "Lasso",
    versions: [Lasso => "Lasso"],
    extensions: [],
}

const LASSO: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r" in Lasso

",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lasso,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lasso,
            syntax: LASSO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LASSO,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r" in Lasso

",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };
