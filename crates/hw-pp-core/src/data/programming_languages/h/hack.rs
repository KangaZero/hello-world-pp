//! Hello-world snippet for `ProgrammingLanguageName::Hack`.
//!
//! Imported from leachim6/hello-world (`h/Hack.hh` at `a152253`),
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
    language: Hack => "Hack",
    versions: [Hack => "Hack"],
    extensions: [Hh => "hh"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hack,
        extensions: &[FileExtension::Hh],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hack,
            syntax: concat!(
                r#"<?hh 
echo ""#,
                output_for_hello_world!(),
                r#"";"#
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, r"<?"),
                (
                    TokenKind::Plain,
                    r"hh 
",
                ),
                (TokenKind::Function, r"echo"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r";"),
            ],
        }],
    };
