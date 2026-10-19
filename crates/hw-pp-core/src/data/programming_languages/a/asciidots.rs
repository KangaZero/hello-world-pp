//! Hello-world snippet for `ProgrammingLanguageName::Asciidots`.
//!
//! Imported from leachim6/hello-world (`a/AsciiDots.ascii` at `a152253`),
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
    language: Asciidots => "AsciiDots",
    versions: [Asciidots => "AsciiDots"],
    extensions: [Ascii => "ascii"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Asciidots,
        extensions: &[FileExtension::Ascii],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Asciidots,
            syntax: concat!(r#".-$""#, output_for_hello_world!(), r#"" "#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#".-$""#, output_for_hello_world!(), r#"" "#),
            )],
        }],
    };
