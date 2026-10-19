//! Hello-world snippet for `ProgrammingLanguageName::Antlang`.
//!
//! Imported from leachim6/hello-world (`a/AntLang.ant` at `a152253`),
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
    language: Antlang => "AntLang",
    versions: [Antlang => "AntLang"],
    extensions: [Ant => "ant"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Antlang,
        extensions: &[FileExtension::Ant],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Antlang,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
