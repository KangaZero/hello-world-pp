//! Hello-world snippet for `ProgrammingLanguageName::Mcsbl`.
//!
//! Imported from the Hello World Collection (`files/MCSBL.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MCSBL.htm>
//! - <http://helloworldcollection.de/#MCSBL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Mcsbl => "MCSBL",
    versions: [Mcsbl => "MCSBL"],
    extensions: [],
}

const MCSBL: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r#" in MCSBL

""#,
    output_for_hello_world!(),
    r#"" println"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mcsbl,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mcsbl,
            syntax: MCSBL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MCSBL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"; ",
                            output_for_hello_world!(),
                            r#" in MCSBL

""#,
                            output_for_hello_world!(),
                            r#"" println"#
                        ),
                    )],
                )
            },
        }],
    };
