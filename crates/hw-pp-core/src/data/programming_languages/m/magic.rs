//! Hello-world snippet for `ProgrammingLanguageName::Magic`.
//!
//! Imported from leachim6/hello-world (`m/Magic` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Magic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Magic => "Magic",
    versions: [Magic => "Magic"],
    extensions: [],
}

const MAGIC: &str = concat!(
    r#"HELLO.WORLD

""#,
    output_for_hello_world!(),
    r#""^#;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Magic,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Magic,
            syntax: MAGIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MAGIC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"HELLO.WORLD

""#,
                            output_for_hello_world!(),
                            r#""^#;"#
                        ),
                    )],
                )
            },
        }],
    };
