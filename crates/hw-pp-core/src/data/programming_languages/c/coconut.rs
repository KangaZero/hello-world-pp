//! Hello-world snippet for `ProgrammingLanguageName::Coconut`.
//!
//! Imported from leachim6/hello-world (`c/Coconut.coco` at `a152253`),
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
    language: Coconut => "Coconut",
    versions: [Coconut => "Coconut"],
    extensions: [Coco => "coco"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Coconut,
        extensions: &[FileExtension::Coco],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Coconut,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#"" |> print"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#"" |> print"#),
            )],
        }],
    };
