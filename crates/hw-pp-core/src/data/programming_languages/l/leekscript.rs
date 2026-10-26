//! Hello-world snippet for `ProgrammingLanguageName::Leekscript`.
//!
//! Imported from leachim6/hello-world (PR #1533 `l/LeekScript.leek`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1533>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Leekscript => "LeekScript",
    versions: [Leekscript => "LeekScript"],
    extensions: [Leek => "leek"],
}

const LEEKSCRIPT: &str = concat!(r#"say(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Leekscript,
        extensions: &[FileExtension::Leek],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Leekscript,
            syntax: LEEKSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LEEKSCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(r#"say(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
