//! Hello-world snippet for `ProgrammingLanguageName::Dc`.
//!
//! Imported from leachim6/hello-world (`d/Dc.dc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dc => "Dc",
    versions: [Dc => "Dc"],
    extensions: [Dc => "dc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dc,
        extensions: &[FileExtension::Dc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dc,
            syntax: concat!(
                r"[",
                output_for_hello_world!(),
                r"
]n"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"[",
                    output_for_hello_world!(),
                    r"
]n"
                ),
            )],
        }],
    };
