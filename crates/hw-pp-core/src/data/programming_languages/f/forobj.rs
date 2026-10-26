//! Hello-world snippet for `ProgrammingLanguageName::Forobj`.
//!
//! Imported from leachim6/hello-world (`f/Forobj.forobj` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Forobj.forobj>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Forobj => "Forobj",
    versions: [Forobj => "Forobj"],
    extensions: [Forobj => "forobj"],
}

const FOROBJ: &str = concat!(r#"""#, output_for_hello_world!(), r#"\n"d"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Forobj,
        extensions: &[FileExtension::Forobj],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Forobj,
            syntax: FOROBJ,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FOROBJ,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"\n"d"#),
                    )],
                )
            },
        }],
    };
