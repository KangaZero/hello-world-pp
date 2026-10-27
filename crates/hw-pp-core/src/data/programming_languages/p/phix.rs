//! Hello-world snippet for `ProgrammingLanguageName::Phix`.
//!
//! Imported from leachim6/hello-world (`p/Phix.exw` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Phix.exw>
//! - <http://phix.x10.mx/docs/html/phix.htm> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Phix => "Phix",
    versions: [Phix => "Phix"],
    extensions: [Exw => "exw"],
}

const PHIX: &str = concat!(r#"puts(1,""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Phix,
        extensions: &[FileExtension::Exw],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Phix,
            syntax: PHIX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PHIX,
                    [(
                        TokenKind::Plain,
                        concat!(r#"puts(1,""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
