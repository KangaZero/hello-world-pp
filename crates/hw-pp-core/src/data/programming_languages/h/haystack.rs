//! Hello-world snippet for `ProgrammingLanguageName::Haystack`.
//!
//! Imported from leachim6/hello-world (`h/haystack.hstk` at `a152253`),
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
    language: Haystack => "haystack",
    versions: [Haystack => "haystack"],
    extensions: [Hstk => "hstk"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Haystack,
        extensions: &[FileExtension::Hstk],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Haystack,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#""o|"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#""o|"#),
            )],
        }],
    };
