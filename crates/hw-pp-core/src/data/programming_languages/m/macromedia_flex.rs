//! Hello-world snippet for `ProgrammingLanguageName::MacromediaFlex`.
//!
//! Imported from the Hello World Collection (`files/Macromedia-Flex.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Macromedia-Flex.txt>
//! - <http://helloworldcollection.de/#Macromedia-Flex>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: MacromediaFlex => "Macromedia-Flex",
    versions: [MacromediaFlex => "Macromedia-Flex"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MacromediaFlex,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MacromediaFlex,
            syntax: concat!(
                r#"<?xml version="1.0" encoding="utf-8"?>
<mx:Application xmlns:mx="http://www.macromedia.com/2003/mxml">
     <!-- Hello Word in Macromedia Flex -->
     <mx:Label text=""#,
                output_for_hello_world!(),
                r#""/>
</mx:Application>"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"<"),
                (TokenKind::Plain, r"?xml version="),
                (TokenKind::String, r#""1.0""#),
                (TokenKind::Plain, r" encoding="),
                (TokenKind::String, r#""utf-8""#),
                (TokenKind::Plain, r"?"),
                (TokenKind::Keyword, r">"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"<mx:Application"),
                (TokenKind::Plain, r" xmlns:mx="),
                (
                    TokenKind::String,
                    r#""http://www.macromedia.com/2003/mxml""#,
                ),
                (TokenKind::Keyword, r">"),
                (
                    TokenKind::Plain,
                    r"
     ",
                ),
                (
                    TokenKind::Comment,
                    r"<!-- Hello Word in Macromedia Flex -->",
                ),
                (
                    TokenKind::Plain,
                    r"
     ",
                ),
                (TokenKind::Keyword, r"<mx:Label"),
                (TokenKind::Plain, r" text="),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Keyword, r"/>"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"</mx:Application>"),
            ],
        }],
    };
