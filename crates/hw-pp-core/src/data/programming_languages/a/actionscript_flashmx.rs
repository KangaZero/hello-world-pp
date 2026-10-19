//! Hello-world snippet for `ProgrammingLanguageName::ActionscriptFlashmx`.
//!
//! Imported from leachim6/hello-world (`a/ActionScript flashmx.as` at `a152253`),
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
    language: ActionscriptFlashmx => "ActionScript flashmx",
    versions: [ActionscriptFlashmx => "ActionScript flashmx"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ActionscriptFlashmx,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ActionscriptFlashmx,
            syntax: concat!(
                r#"_root.createTextField("mytext",1,100,100,300,100);
mytext.multiline = true;
mytext.wordWrap = true;
mytext.border = false;

myformat = new TextFormat();
myformat.color = 0xff0000;
myformat.bullet = false;
myformat.underline = true;

mytext.text = ""#,
                output_for_hello_world!(),
                r#"";
mytext.setTextFormat(myformat);"#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, r"_root."),
                (TokenKind::Function, r"createTextField"),
                (TokenKind::Plain, r"("),
                (TokenKind::String, r#""mytext""#),
                (TokenKind::Plain, r","),
                (TokenKind::Number, r"1"),
                (TokenKind::Plain, r","),
                (TokenKind::Number, r"100"),
                (TokenKind::Plain, r","),
                (TokenKind::Number, r"100"),
                (TokenKind::Plain, r","),
                (TokenKind::Number, r"300"),
                (TokenKind::Plain, r","),
                (TokenKind::Number, r"100"),
                (
                    TokenKind::Plain,
                    r");
mytext.",
                ),
                (TokenKind::Function, r"multiline"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"="),
                (
                    TokenKind::Plain,
                    r" true;
mytext.",
                ),
                (TokenKind::Function, r"wordWrap"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"="),
                (
                    TokenKind::Plain,
                    r" true;
mytext.",
                ),
                (TokenKind::Function, r"border"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"="),
                (
                    TokenKind::Plain,
                    r" false;

myformat ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"new"),
                (
                    TokenKind::Plain,
                    r" TextFormat();
myformat.",
                ),
                (TokenKind::Function, r"color"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0xff0000"),
                (
                    TokenKind::Plain,
                    r";
myformat.",
                ),
                (TokenKind::Function, r"bullet"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"="),
                (
                    TokenKind::Plain,
                    r" false;
myformat.",
                ),
                (TokenKind::Function, r"underline"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"="),
                (
                    TokenKind::Plain,
                    r" true;

mytext.",
                ),
                (TokenKind::Function, r"text"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (
                    TokenKind::Plain,
                    r";
mytext.",
                ),
                (TokenKind::Function, r"setTextFormat"),
                (TokenKind::Plain, r"(myformat);"),
            ],
        }],
    };
