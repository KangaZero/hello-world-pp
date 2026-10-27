//! Hello-world snippet for `ProgrammingLanguageName::Rebmu`.
//!
//! Imported from leachim6/hello-world (`r/REBMU.reb` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/REBMU.reb>
//! - <https://en.wikipedia.org/wiki/Rebol> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Imperative, PrototypeBased};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rebmu => "REBMU",
    versions: [Rebmu => "REBMU"],
    extensions: [Reb => "reb"],
}

const REBMU: &str = concat!(r#"rebmu [p""#, output_for_hello_world!(), r#""]"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rebmu,
        extensions: &[FileExtension::Reb],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, PrototypeBased, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rebmu,
            syntax: REBMU,
            tokenized_syntax: &const {
                checked_token_lengths(
                    REBMU,
                    [(
                        TokenKind::Plain,
                        concat!(r#"rebmu [p""#, output_for_hello_world!(), r#""]"#),
                    )],
                )
            },
        }],
    };
