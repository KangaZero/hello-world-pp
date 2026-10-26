//! Hello-world snippet for `ProgrammingLanguageName::Tkinter`.
//!
//! Imported from leachim6/hello-world (`t/Tkinter.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Tkinter.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Python, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Tkinter => "Tkinter",
    versions: [Tkinter => "Tkinter"],
    extensions: [],
}

const TKINTER: &str = concat!(
    r#"import tkinter as tk
root = tk.Tk()
greeting = tk.Label(text=""#,
    output_for_hello_world!(),
    r#"")
greeting.pack()
root.mainloop()"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tkinter,
        extensions: &[],
        categories: &[Framework],
        families: &[Python],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tkinter,
            syntax: TKINTER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TKINTER,
                    [
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" tkinter "),
                        (TokenKind::Keyword, r"as"),
                        (
                            TokenKind::Plain,
                            r" tk
root ",
                        ),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" tk"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"Tk"),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"
greeting ",
                        ),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" tk"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"Label"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"text"),
                        (TokenKind::Keyword, r"="),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
greeting",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"pack"),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"
root",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"mainloop"),
                        (TokenKind::Punctuation, r"()"),
                    ],
                )
            },
        }],
    };
