//! Hello-world snippet for `ProgrammingLanguageName::Wxpython`.
//!
//! Imported from leachim6/hello-world (`w/wxPython.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/wxPython.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Python, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Wxpython => "wxPython",
    versions: [Wxpython => "wxPython"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wxpython,
        extensions: &[],
        categories: &[Framework],
        families: &[Python],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wxpython,
            syntax: concat!(
                r#"import wx 
 
app = wx.App() 
window = wx.Frame(None)
panel = wx.Panel(window) 
label = wx.StaticText(panel, label = ""#,
                output_for_hello_world!(),
                r#"")
window.Show(True) 
app.MainLoop()"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" wx 
 
app ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" wx"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"App"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r" 
window ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" wx"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"Frame"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"None"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
panel ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" wx"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"Panel"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"window"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r" 
label ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" wx"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"StaticText"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"panel"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" label "),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
window",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"Show"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"True"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r" 
app",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"MainLoop"),
                (TokenKind::Punctuation, r"()"),
            ],
        }],
    };
