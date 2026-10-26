//! Hello-world snippet for `ProgrammingLanguageName::Rascal`.
//!
//! Imported from leachim6/hello-world (PR #1664 `r/rascal.rsc`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1664>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rascal => "Rascal",
    versions: [Rascal => "Rascal"],
    extensions: [Rsc => "rsc"],
}

const RASCAL: &str = concat!(r#"println(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rascal,
        extensions: &[FileExtension::Rsc],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rascal,
            syntax: RASCAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RASCAL,
                    [(
                        TokenKind::Plain,
                        concat!(r#"println(""#, output_for_hello_world!(), r#"");"#),
                    )],
                )
            },
        }],
    };
