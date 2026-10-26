//! Hello-world snippet for `ProgrammingLanguageName::Supercollider`.
//!
//! Imported from leachim6/hello-world (`s/SuperCollider.sc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SuperCollider.sc>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Music, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Supercollider => "SuperCollider",
    versions: [Supercollider => "SuperCollider"],
    extensions: [],
}

const SUPERCOLLIDER: &str = concat!(r#"""#, output_for_hello_world!(), r#"".postln;"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Supercollider,
        extensions: &[],
        categories: &[Music],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Supercollider,
            syntax: SUPERCOLLIDER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SUPERCOLLIDER,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"".postln;"#),
                    )],
                )
            },
        }],
    };
