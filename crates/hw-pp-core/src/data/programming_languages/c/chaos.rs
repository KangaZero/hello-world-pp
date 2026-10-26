//! Hello-world snippet for `ProgrammingLanguageName::Chaos`.
//!
//! Imported from leachim6/hello-world (`c/Chaos.kaos` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Chaos.kaos>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Chaos => "Chaos",
    versions: [Chaos => "Chaos"],
    extensions: [Kaos => "kaos"],
}

const CHAOS: &str = concat!(r#"print ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Chaos,
        extensions: &[FileExtension::Kaos],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Chaos,
            syntax: CHAOS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CHAOS,
                    [(
                        TokenKind::Plain,
                        concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
