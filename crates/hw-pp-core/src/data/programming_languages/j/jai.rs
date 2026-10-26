//! Hello-world snippet for `ProgrammingLanguageName::Jai`.
//!
//! Imported from leachim6/hello-world (`j/Jai.jai` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/Jai.jai>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Jai => "Jai",
    versions: [Jai => "Jai"],
    extensions: [Jai => "jai"],
}

const JAI: &str = concat!(
    r#"#import "Print"

main::(){

	print(""#,
    output_for_hello_world!(),
    r#"");

}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jai,
        extensions: &[FileExtension::Jai],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jai,
            syntax: JAI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JAI,
                    [
                        (TokenKind::Macro, r#"#import "Print""#),
                        (
                            TokenKind::Plain,
                            r"

main",
                        ),
                        (TokenKind::Punctuation, r"::(){"),
                        (
                            TokenKind::Plain,
                            r"

	",
                        ),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
