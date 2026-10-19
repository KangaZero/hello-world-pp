//! Hello-world snippet for `ProgrammingLanguageName::Beanshell`.
//!
//! Imported from leachim6/hello-world (`b/Beanshell.bsh` at `a152253`),
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
    language: Beanshell => "Beanshell",
    versions: [Beanshell => "Beanshell"],
    extensions: [Bsh => "bsh"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Beanshell,
        extensions: &[FileExtension::Bsh],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Beanshell,
            syntax: concat!(r#"print (""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[
                (TokenKind::Function, r"print"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r");"),
            ],
        }],
    };
