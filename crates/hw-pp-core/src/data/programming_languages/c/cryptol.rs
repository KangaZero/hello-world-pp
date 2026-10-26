//! Hello-world snippet for `ProgrammingLanguageName::Cryptol`.
//!
//! Imported from leachim6/hello-world (`c/Cryptol` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cryptol>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cryptol => "Cryptol",
    versions: [Cryptol => "Cryptol"],
    extensions: [],
}

const CRYPTOL: &str = concat!(
    r#":set ascii=on
""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cryptol,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cryptol,
            syntax: CRYPTOL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CRYPTOL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#":set ascii=on
""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
