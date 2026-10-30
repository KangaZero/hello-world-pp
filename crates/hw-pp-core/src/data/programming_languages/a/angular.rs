//! Hello-world snippet for `ProgrammingLanguageName::Angular`.
//!
//! Imported from leachim6/hello-world (`a/Angular.js` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Angular.js>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Angular => "Angular",
    versions: [Angular => "Angular"],
    extensions: [],
}

const ANGULAR: &str = concat!(
    r#"$scope.$log = $log;
$scope.message = ""#,
    output_for_hello_world!(),
    r#"";"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Angular,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Angular,
            syntax: ANGULAR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ANGULAR,
                    [
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
                )
            },
        }],
    };
