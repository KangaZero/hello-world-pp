//! Hello-world snippet for `ProgrammingLanguageName::Context`.
//!
//! Imported from the Hello World Collection (`files/ConTeXt.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ConTeXt.htm>
//! - <http://helloworldcollection.de/#ConTeXt>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Context => "ConTeXt",
    versions: [Context => "ConTeXt"],
    extensions: [],
}

const CONTEXT: &str = concat!(
    r"% ",
    output_for_hello_world!(),
    r" in ConTeXt

\starttext
",
    output_for_hello_world!(),
    r"
\stoptext"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Context,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Context,
            syntax: CONTEXT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CONTEXT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"% ",
                            output_for_hello_world!(),
                            r" in ConTeXt

\starttext
",
                            output_for_hello_world!(),
                            r"
\stoptext"
                        ),
                    )],
                )
            },
        }],
    };
