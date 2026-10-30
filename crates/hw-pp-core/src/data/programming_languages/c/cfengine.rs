//! Hello-world snippet for `ProgrammingLanguageName::Cfengine`.
//!
//! Imported from leachim6/hello-world (`c/CFEngine.cf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CFEngine.cf>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Configuration,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Cfengine => "CFEngine",
    versions: [Cfengine => "CFEngine"],
    extensions: [Cf => "cf"],
}

const CFENGINE: &str = concat!(
    r#"body common control
{
  bundlesequence => { "run" };
}

bundle agent run
{
  reports:
    cfengine::
      ""#,
    output_for_hello_world!(),
    r#"";
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cfengine,
        extensions: &[FileExtension::Cf],
        categories: &[Configuration],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cfengine,
            syntax: CFENGINE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CFENGINE,
                    [
                        (
                            TokenKind::Plain,
                            r"body common control
",
                        ),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  bundlesequence => ",
                        ),
                        (TokenKind::Punctuation, r"{"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""run""#),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"};"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"}"),
                        (
                            TokenKind::Plain,
                            r"

bundle agent run
",
                        ),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  reports",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
    cfengine",
                        ),
                        (TokenKind::Punctuation, r"::"),
                        (
                            TokenKind::Plain,
                            r"
      ",
                        ),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r";"),
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
