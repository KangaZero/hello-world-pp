//! Hello-world snippet for `ProgrammingLanguageName::Pyqt5`.
//!
//! Imported from leachim6/hello-world (`p/PyQt5.py` at `a152253`),
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
    language: Pyqt5 => "PyQt5",
    versions: [Pyqt5 => "PyQt5"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pyqt5,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pyqt5,
            syntax: concat!(
                r#"import sys
from PyQt5.QtWidgets import QApplication, QWidget, QLabel

app = QApplication(sys.argv)
widget = QWidget()

helloWorld = QLabel(widget)
helloWorld.setText(""#,
                output_for_hello_world!(),
                r#"")

widget.show()
sys.exit(app.exec_())"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" sys
",
                ),
                (TokenKind::Keyword, r"from"),
                (TokenKind::Plain, r" PyQt5"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"QtWidgets "),
                (TokenKind::Keyword, r"import"),
                (TokenKind::Plain, r" QApplication"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" QWidget"),
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
widget ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"QWidget"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"

helloWorld ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"QLabel"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"widget"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
helloWorld",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"setText"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"

widget",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"show"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"
sys",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"exit"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"app"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"exec_"),
                (TokenKind::Punctuation, r"())"),
            ],
        }],
    };
