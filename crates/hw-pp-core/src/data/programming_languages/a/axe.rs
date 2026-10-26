//! Hello-world snippet for `ProgrammingLanguageName::Axe`.
//!
//! Imported from leachim6/hello-world (`a/Axe.axe` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Axe.axe>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Axe => "Axe",
    versions: [Axe => "Axe"],
    extensions: [Axe => "axe"],
}

const AXE: &str = concat!(r#"Disp ""#, output_for_hello_world!(), r#"",i"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Axe,
        extensions: &[FileExtension::Axe],
        categories: &[Compiled],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Axe,
            syntax: AXE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AXE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"Disp ""#, output_for_hello_world!(), r#"",i"#),
                    )],
                )
            },
        }],
    };
