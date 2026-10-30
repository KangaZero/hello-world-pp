//! Hello-world snippet for `ProgrammingLanguageName::Dg`.
//!
//! Imported from leachim6/hello-world (`d/dg.dg` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/dg.dg>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Dg => "dg",
    versions: [Dg => "dg"],
    extensions: [Dg => "dg"],
}

const DG: &str = concat!(r#"print ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dg,
        extensions: &[FileExtension::Dg],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dg,
            syntax: DG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DG,
                    [(
                        TokenKind::Plain,
                        concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
