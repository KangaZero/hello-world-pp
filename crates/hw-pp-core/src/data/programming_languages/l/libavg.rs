//! Hello-world snippet for `ProgrammingLanguageName::Libavg`.
//!
//! Imported from leachim6/hello-world (`l/libavg.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/libavg.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Libavg => "libavg",
    versions: [Libavg => "libavg"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Libavg,
        extensions: &[],
        categories: &[Framework],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Libavg,
            syntax: concat!(
                r#"from libavg import avg

player = avg.Player.get()
canvas = player.createMainCanvas(size=(640,480))
rootNode = canvas.getRootNode()
avg.WordsNode(pos=(10,10), font="arial", 
        text=""#,
                output_for_hello_world!(),
                r#"", parent=rootNode)
player.play()"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"from"),
                (TokenKind::Plain, r" libavg "),
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" avg

player ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" avg"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"Player"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"get"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"
canvas ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" player"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"createMainCanvas"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"size"),
                (TokenKind::Keyword, r"="),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Number, r"640"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Number, r"480"),
                (TokenKind::Punctuation, r"))"),
                (
                    TokenKind::Plain,
                    r"
rootNode ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" canvas"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"getRootNode"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"
avg",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"WordsNode"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"pos"),
                (TokenKind::Keyword, r"="),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Number, r"10"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Number, r"10"),
                (TokenKind::Punctuation, r"),"),
                (TokenKind::Plain, r" font"),
                (TokenKind::Keyword, r"="),
                (TokenKind::String, r#""arial""#),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" 
        text",
                ),
                (TokenKind::Keyword, r"="),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" parent"),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r"rootNode"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
player",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"play"),
                (TokenKind::Punctuation, r"()"),
            ],
        }],
    };
