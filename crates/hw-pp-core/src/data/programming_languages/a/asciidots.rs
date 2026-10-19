//! Hello-world snippet for `ProgrammingLanguageName::Asciidots`.
//!
//! Imported from leachim6/hello-world (`a/AsciiDots.ascii` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/AsciiDots.ascii>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Asciidots => "AsciiDots",
    versions: [Asciidots => "AsciiDots"],
    extensions: [Ascii => "ascii"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Asciidots,
        extensions: &[FileExtension::Ascii],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Asciidots,
            syntax: concat!(r#".-$""#, output_for_hello_world!(), r#"" "#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#".-$""#, output_for_hello_world!(), r#"" "#),
            )],
        }],
    };
