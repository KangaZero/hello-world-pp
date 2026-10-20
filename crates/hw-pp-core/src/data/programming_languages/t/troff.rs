//! Hello-world snippet for `ProgrammingLanguageName::Troff`.
//!
//! Imported from the Hello World Collection (`files/troff.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/troff.txt>
//! - <http://helloworldcollection.de/#troff>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Troff => "troff",
    versions: [Troff => "troff"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Troff,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Troff,
            syntax: concat!(
                r#"\"	""#,
                output_for_hello_world!(),
                r#"" in troff

"#,
                output_for_hello_world!()
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"\"	""#,
                    output_for_hello_world!(),
                    r#"" in troff

"#,
                    output_for_hello_world!()
                ),
            )],
        }],
    };
