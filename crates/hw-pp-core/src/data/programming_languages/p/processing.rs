//! Hello-world snippet for `ProgrammingLanguageName::Processing`.
//!
//! Imported from leachim6/hello-world (`p/Processing.pde` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Processing.pde>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Java, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Processing => "Processing",
    versions: [Processing => "Processing"],
    extensions: [Pde => "pde"],
}

const PROCESSING: &str = concat!(
    r#"size(128, 128);
background(0);
textAlign(CENTER, CENTER);
fill(255);
text(""#,
    output_for_hello_world!(),
    r#"", width / 2, height / 2);"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Processing,
        extensions: &[FileExtension::Pde],
        categories: &[Educational],
        families: &[Java],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Processing,
            syntax: PROCESSING,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PROCESSING,
                    [
                        (TokenKind::Function, r"size"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Number, r"128"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"128"),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Function, r"background"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Function, r"textAlign"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"CENTER"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" CENTER"),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Function, r"fill"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Number, r"255"),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Function, r"text"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" width / "),
                        (TokenKind::Number, r"2"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" height / "),
                        (TokenKind::Number, r"2"),
                        (TokenKind::Punctuation, r");"),
                    ],
                )
            },
        }],
    };
