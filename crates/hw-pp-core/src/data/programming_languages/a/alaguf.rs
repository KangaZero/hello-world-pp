//! Hello-world snippet for `ProgrammingLanguageName::Alaguf`.
//!
//! Imported from leachim6/hello-world (`a/ALAGUF.alaguf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ALAGUF.alaguf>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Alaguf => "ALAGUF",
    versions: [Alaguf => "ALAGUF"],
    extensions: [Alaguf => "alaguf"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Alaguf,
        extensions: &[FileExtension::Alaguf],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Alaguf,
            syntax: concat!(r#"""#, output_for_hello_world!(), r##""#!"##),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r##""#!"##),
            )],
        }],
    };
