//! Hello-world snippet for `ProgrammingLanguageName::GoPlus`.
//!
//! Imported from leachim6/hello-world (`g/Go+.gop` at `a152253`),
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
    language: GoPlus => "Go+",
    versions: [GoPlus => "Go+"],
    extensions: [Gop => "gop"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GoPlus,
        extensions: &[FileExtension::Gop],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GoPlus,
            syntax: concat!(r#"println ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"println ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
