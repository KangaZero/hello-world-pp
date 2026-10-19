//! Hello-world snippet for `ProgrammingLanguageName::Streem`.
//!
//! Imported from leachim6/hello-world (`s/Streem.strm` at `a152253`),
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
    language: Streem => "Streem",
    versions: [Streem => "Streem"],
    extensions: [Strm => "strm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Streem,
        extensions: &[FileExtension::Strm],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Streem,
            syntax: concat!(r#"[""#, output_for_hello_world!(), r#""] | stdout"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"[""#, output_for_hello_world!(), r#""] | stdout"#),
            )],
        }],
    };
