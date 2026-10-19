//! Hello-world snippet for `ProgrammingLanguageName::Brzrk`.
//!
//! Imported from leachim6/hello-world (`b/BRZRK.brzrk` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/BRZRK.brzrk>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Brzrk => "BRZRK",
    versions: [Brzrk => "BRZRK"],
    extensions: [Brzrk => "brzrk"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brzrk,
        extensions: &[FileExtension::Brzrk],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brzrk,
            syntax: concat!(r#"$""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"$""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
