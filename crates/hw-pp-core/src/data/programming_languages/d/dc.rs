//! Hello-world snippet for `ProgrammingLanguageName::Dc`.
//!
//! Imported from leachim6/hello-world (`d/Dc.dc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Dc.dc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Dc => "Dc",
    versions: [Dc => "Dc"],
    extensions: [Dc => "dc"],
}

const DC: &str = concat!(
    r"[",
    output_for_hello_world!(),
    r"
]n"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dc,
        extensions: &[FileExtension::Dc],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dc,
            syntax: DC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"[",
                            output_for_hello_world!(),
                            r"
]n"
                        ),
                    )],
                )
            },
        }],
    };
