//! Hello-world snippet for `ProgrammingLanguageName::Dascript`.
//!
//! Imported from leachim6/hello-world (`d/daScript.das` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/daScript.das>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Dascript => "daScript",
    versions: [Dascript => "daScript"],
    extensions: [Das => "das"],
}

const DASCRIPT: &str = concat!(
    r#"[export]
def main
	print(""#,
    output_for_hello_world!(),
    r#"\n")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dascript,
        extensions: &[FileExtension::Das],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dascript,
            syntax: DASCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DASCRIPT,
                    [
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Keyword, r"export"),
                        (TokenKind::Punctuation, r"]"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"def"),
                        (
                            TokenKind::Plain,
                            r" main
	",
                        ),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
