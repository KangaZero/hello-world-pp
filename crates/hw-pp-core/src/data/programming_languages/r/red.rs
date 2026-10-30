//! Hello-world snippet for `ProgrammingLanguageName::Red`.
//!
//! Imported from leachim6/hello-world (`r/Red.red` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Red.red>
//! - <https://www.wikidata.org/wiki/Q3217187> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Imperative};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Red => "Red",
    versions: [Red => "Red"],
    extensions: [Red => "red"],
}

const RED: &str = concat!(
    r#"Red [Needs: 'View]
view [text ""#,
    output_for_hello_world!(),
    r#""] "#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Red,
        extensions: &[FileExtension::Red],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Red,
            syntax: RED,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RED,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"Red [Needs: 'View]
view [text ""#,
                            output_for_hello_world!(),
                            r#""] "#
                        ),
                    )],
                )
            },
        }],
    };
