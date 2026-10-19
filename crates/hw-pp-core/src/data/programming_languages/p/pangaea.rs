//! Hello-world snippet for `ProgrammingLanguageName::Pangaea`.
//!
//! Imported from leachim6/hello-world (`p/Pangaea.pangaea` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pangaea.pangaea>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Pangaea => "Pangaea",
    versions: [Pangaea => "Pangaea"],
    extensions: [Pangaea => "pangaea"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pangaea,
        extensions: &[FileExtension::Pangaea],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pangaea,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#"".p"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#"".p"#),
            )],
        }],
    };
