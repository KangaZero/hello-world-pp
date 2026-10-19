//! Hello-world snippet for `ProgrammingLanguageName::Pyside2`.
//!
//! Imported from leachim6/hello-world (`p/PySide2.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PySide2.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Pyside2 => "PySide2",
    versions: [Pyside2 => "PySide2"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pyside2,
        extensions: &[],
        categories: &[Framework],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pyside2,
            syntax: concat!(
                r#"import sys
from PySide2.QtWidgets import QApplication, QLabel

app = QApplication(sys.argv)
label = QLabel(""#,
                output_for_hello_world!(),
                r#"")
label.show()
app.exec_()"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" sys
",
                ),
                (TokenKind::Keyword, r"from"),
                (TokenKind::Plain, r" PySide2"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"QtWidgets "),
                (TokenKind::Keyword, r"import"),
                (TokenKind::Plain, r" QApplication"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" QLabel

app ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"QApplication"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"sys"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"argv"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
label ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"QLabel"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
label",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"show"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"
app",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"exec_"),
                (TokenKind::Punctuation, r"()"),
            ],
        }],
    };
