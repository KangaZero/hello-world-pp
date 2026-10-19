//! Hello-world snippet for `ProgrammingLanguageName::SmallworldMagik`.
//!
//! Imported from leachim6/hello-world (`s/Smallworld MAGIK.magik` at `a152253`),
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
    language: SmallworldMagik => "Smallworld MAGIK",
    versions: [SmallworldMagik => "Smallworld MAGIK"],
    extensions: [Magik => "magik"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SmallworldMagik,
        extensions: &[FileExtension::Magik],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SmallworldMagik,
            syntax: concat!(r#"write(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"write(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
