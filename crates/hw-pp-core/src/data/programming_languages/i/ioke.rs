//! Hello-world snippet for `ProgrammingLanguageName::Ioke`.
//!
//! Imported from leachim6/hello-world (`i/Ioke.ik` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/Ioke.ik>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ioke => "Ioke",
    versions: [Ioke => "Ioke"],
    extensions: [Ik => "ik"],
}

const IOKE: &str = concat!(r#"""#, output_for_hello_world!(), r#"" println"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ioke,
        extensions: &[FileExtension::Ik],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ioke,
            syntax: IOKE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    IOKE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"" println"#),
                    )],
                )
            },
        }],
    };
