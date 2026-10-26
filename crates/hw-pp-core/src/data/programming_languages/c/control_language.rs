//! Hello-world snippet for `ProgrammingLanguageName::ControlLanguage`.
//!
//! Imported from leachim6/hello-world (`c/Control Language.cllc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Control%20Language.cllc>
//! - <https://www.wikidata.org/wiki/Q263490> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: ControlLanguage => "Control Language",
    versions: [ControlLanguage => "Control Language"],
    extensions: [Cllc => "cllc"],
}

const CONTROL_LANGUAGE: &str = concat!(r#"SNDPGMMSG MSG(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ControlLanguage,
        extensions: &[FileExtension::Cllc],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ControlLanguage,
            syntax: CONTROL_LANGUAGE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CONTROL_LANGUAGE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"SNDPGMMSG MSG(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
