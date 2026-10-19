//! Hello-world snippet for `ProgrammingLanguageName::Mokkosu`.
//!
//! Imported from leachim6/hello-world (`m/Mokkosu.mok` at `a152253`),
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
    language: Mokkosu => "Mokkosu",
    versions: [Mokkosu => "Mokkosu"],
    extensions: [Mok => "mok"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mokkosu,
        extensions: &[FileExtension::Mok],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mokkosu,
            syntax: concat!(r#"do println ""#, output_for_hello_world!(), r#"";"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"do println ""#, output_for_hello_world!(), r#"";"#),
            )],
        }],
    };
