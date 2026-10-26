//! Hello-world snippet for `ProgrammingLanguageName::Ambienttalk`.
//!
//! Imported from leachim6/hello-world (PR #1662 `a/AmbientTalk.at`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1662>
//! - <https://www.wikidata.org/wiki/Q4741415> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Concurrent, EventDriven, ObjectOriented, PrototypeBased, Reflective,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ambienttalk => "AmbientTalk",
    versions: [Ambienttalk => "AmbientTalk"],
    extensions: [At => "at"],
}

const AMBIENTTALK: &str = concat!(r#"system.println(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ambienttalk,
        extensions: &[FileExtension::At],
        categories: &[Scripting],
        families: &[],
        paradigms: &[
            ObjectOriented,
            PrototypeBased,
            Concurrent,
            Reflective,
            EventDriven,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ambienttalk,
            syntax: AMBIENTTALK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AMBIENTTALK,
                    [(
                        TokenKind::Plain,
                        concat!(r#"system.println(""#, output_for_hello_world!(), r#"");"#),
                    )],
                )
            },
        }],
    };
