//! Hello-world snippet for `ProgrammingLanguageName::Brzrk`.
//!
//! Imported from leachim6/hello-world (`b/BRZRK.brzrk` at `a152253`),
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
    language: Brzrk => "BRZRK",
    versions: [Brzrk => "BRZRK"],
    extensions: [Brzrk => "brzrk"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brzrk,
        extensions: &[FileExtension::Brzrk],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brzrk,
            syntax: concat!(r#"$""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"$""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
