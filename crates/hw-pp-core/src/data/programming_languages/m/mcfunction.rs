//! Hello-world snippet for `ProgrammingLanguageName::Mcfunction`.
//!
//! Imported from leachim6/hello-world (PR #1670 `m/Mcfunction.mcfunction`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1670>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Mcfunction => "Mcfunction",
    versions: [Mcfunction => "Mcfunction"],
    extensions: [Mcfunction => "mcfunction"],
}

const MCFUNCTION: &str = concat!(r"say ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mcfunction,
        extensions: &[FileExtension::Mcfunction],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mcfunction,
            syntax: MCFUNCTION,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MCFUNCTION,
                    [(
                        TokenKind::Plain,
                        concat!(r"say ", output_for_hello_world!()),
                    )],
                )
            },
        }],
    };
