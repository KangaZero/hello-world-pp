//! Hello-world snippet for `ProgrammingLanguageName::SmallworldMagik`.
//!
//! Imported from leachim6/hello-world (`s/Smallworld MAGIK.magik` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Smallworld%20MAGIK.magik>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: SmallworldMagik => "Smallworld MAGIK",
    versions: [SmallworldMagik => "Smallworld MAGIK"],
    extensions: [Magik => "magik"],
}

const SMALLWORLD_MAGIK: &str = concat!(r#"write(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SmallworldMagik,
        extensions: &[FileExtension::Magik],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SmallworldMagik,
            syntax: SMALLWORLD_MAGIK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SMALLWORLD_MAGIK,
                    [(
                        TokenKind::Plain,
                        concat!(r#"write(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
