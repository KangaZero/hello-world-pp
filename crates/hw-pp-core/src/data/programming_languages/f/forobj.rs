//! Hello-world snippet for `ProgrammingLanguageName::Forobj`.
//!
//! Imported from leachim6/hello-world (`f/Forobj.forobj` at `a152253`),
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
    language: Forobj => "Forobj",
    versions: [Forobj => "Forobj"],
    extensions: [Forobj => "forobj"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Forobj,
        extensions: &[FileExtension::Forobj],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Forobj,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#"\n"d"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#"\n"d"#),
            )],
        }],
    };
