//! Hello-world snippet for `ProgrammingLanguageName::Angular`.
//!
//! Imported from leachim6/hello-world (`a/Angular.js` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Angular => "Angular",
    versions: [Angular => "Angular"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Angular,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Angular,
            syntax: concat!(
                r#"$scope.$log = $log;
$scope.message = ""#,
                output_for_hello_world!(),
                r#"";"#
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, r"$"),
                (TokenKind::Plain, r"scope"),
                (TokenKind::Punctuation, r".$"),
                (TokenKind::Plain, r"log "),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"$"),
                (TokenKind::Plain, r"log"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"$"),
                (TokenKind::Plain, r"scope"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"message "),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r";"),
            ],
        }],
    };
