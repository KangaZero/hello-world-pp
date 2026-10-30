//! Hello-world snippet for `ProgrammingLanguageName::Filemaker`.
//!
//! Imported from the Hello World Collection (`files/Filemaker.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Filemaker.txt>
//! - <http://helloworldcollection.de/#Filemaker>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Filemaker => "Filemaker",
    versions: [Filemaker => "Filemaker"],
    extensions: [],
}

const FILEMAKER: &str = concat!(
    r"#",
    output_for_hello_world!(),
    r#" in Filemaker Script
Show Custom Dialog [""#,
    output_for_hello_world!(),
    r#"" ; ""#,
    output_for_hello_world!(),
    r#""]"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Filemaker,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Filemaker,
            syntax: FILEMAKER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FILEMAKER,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"#",
                            output_for_hello_world!(),
                            r#" in Filemaker Script
Show Custom Dialog [""#,
                            output_for_hello_world!(),
                            r#"" ; ""#,
                            output_for_hello_world!(),
                            r#""]"#
                        ),
                    )],
                )
            },
        }],
    };
