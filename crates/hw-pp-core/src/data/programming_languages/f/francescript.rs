//! Hello-world snippet for `ProgrammingLanguageName::Francescript`.
//!
//! Imported from leachim6/hello-world (PR #1482 `f/FranceScript.fr`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1482>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Francescript => "FranceScript",
    versions: [Francescript => "FranceScript"],
    extensions: [Fr => "fr"],
}

const FRANCESCRIPT: &str = concat!(
    r#"console->log(""#,
    output_for_hello_world!(),
    r#" !") fin"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Francescript,
        extensions: &[FileExtension::Fr],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Francescript,
            syntax: FRANCESCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FRANCESCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"console->log(""#,
                            output_for_hello_world!(),
                            r#" !") fin"#
                        ),
                    )],
                )
            },
        }],
    };
