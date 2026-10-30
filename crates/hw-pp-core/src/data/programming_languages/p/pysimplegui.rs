//! Hello-world snippet for `ProgrammingLanguageName::Pysimplegui`.
//!
//! Imported from leachim6/hello-world (`p/PySimpleGUI.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PySimpleGUI.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Pysimplegui => "PySimpleGUI",
    versions: [Pysimplegui => "PySimpleGUI"],
    extensions: [],
}

const PYSIMPLEGUI: &str = concat!(
    r#"import PySimpleGUI

PySimpleGUI.popup_no_buttons(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pysimplegui,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pysimplegui,
            syntax: PYSIMPLEGUI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PYSIMPLEGUI,
                    [
                        (TokenKind::Keyword, r"import"),
                        (
                            TokenKind::Plain,
                            r" PySimpleGUI

PySimpleGUI",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"popup_no_buttons"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
