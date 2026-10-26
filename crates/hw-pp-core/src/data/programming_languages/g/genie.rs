//! Hello-world snippet for `ProgrammingLanguageName::Genie`.
//!
//! Imported from leachim6/hello-world (`g/Genie.gs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Genie.gs>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Genie => "Genie",
    versions: [Genie => "Genie"],
    extensions: [Gs => "gs"],
}

const GENIE: &str = concat!(
    r#"[indent=2]
init
  print ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Genie,
        extensions: &[FileExtension::Gs],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Genie,
            syntax: GENIE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GENIE,
                    [
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Plain, r"indent="),
                        (TokenKind::Number, r"2"),
                        (TokenKind::Punctuation, r"]"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"init"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
