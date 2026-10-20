//! Hello-world snippet for `ProgrammingLanguageName::Pyqt4`.
//!
//! Imported from leachim6/hello-world (`p/PyQt4.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PyQt4.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Pyqt4 => "PyQt4",
    versions: [Pyqt4 => "PyQt4"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pyqt4,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pyqt4,
            syntax: concat!(
                r#"import sys
from PyQt4 import QtGui

app = QtGui.QApplication(sys.argv)
widget = QtGui.QWidget()
helloWorld = QtGui.QLabel(widget)
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
                (TokenKind::Plain, r" PyQt4 "),
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" QtGui

app ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" QtGui"),
                (TokenKind::Punctuation, r"."),
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
                (TokenKind::Plain, r" QtGui"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"QWidget"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"
helloWorld ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" QtGui"),
                (TokenKind::Punctuation, r"."),
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
