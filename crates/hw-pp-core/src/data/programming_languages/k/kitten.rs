//! Hello-world snippet for `ProgrammingLanguageName::Kitten`.
//!
//! Imported from leachim6/hello-world (`k/Kitten.ktn` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/Kitten.ktn>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Kitten => "Kitten",
    versions: [Kitten => "Kitten"],
    extensions: [],
}

const KITTEN: &str = concat!(r#"""#, output_for_hello_world!(), r#"" say"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kitten,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kitten,
            syntax: KITTEN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    KITTEN,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"" say"#),
                    )],
                )
            },
        }],
    };
