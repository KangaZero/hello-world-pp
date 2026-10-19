//! Hello-world snippet for `ProgrammingLanguageName::Fancy`.
//!
//! Imported from leachim6/hello-world (`f/Fancy.fy` at `a152253`),
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
    language: Fancy => "Fancy",
    versions: [Fancy => "Fancy"],
    extensions: [Fy => "fy"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fancy,
        extensions: &[FileExtension::Fy],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fancy,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#"" println "#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#"" println "#),
            )],
        }],
    };
