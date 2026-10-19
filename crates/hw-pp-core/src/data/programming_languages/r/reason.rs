//! Hello-world snippet for `ProgrammingLanguageName::Reason`.
//!
//! Imported from leachim6/hello-world (`r/Reason.re` at `a152253`),
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
    language: Reason => "Reason",
    versions: [Reason => "Reason"],
    extensions: [Re => "re"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Reason,
        extensions: &[FileExtension::Re],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Reason,
            syntax: concat!(r#"print_string ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print_string ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
