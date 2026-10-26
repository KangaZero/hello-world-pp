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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Troff => "troff",
    versions: [Troff => "troff"],
    extensions: [],
}

const TROFF: &str = concat!(
    r#"\"	""#,
    output_for_hello_world!(),
    r#"" in troff

"#,
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Troff,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Troff,
            syntax: TROFF,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TROFF,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"\"	""#,
                            output_for_hello_world!(),
                            r#"" in troff

"#,
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };
