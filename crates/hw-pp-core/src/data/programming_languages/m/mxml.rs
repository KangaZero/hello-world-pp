//! Hello-world snippet for `ProgrammingLanguageName::Mxml`.
//!
//! Imported from leachim6/hello-world (`m/MXML.mxml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/MXML.mxml>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Mxml => "MXML",
    versions: [Mxml => "MXML"],
    extensions: [Mxml => "mxml"],
}

const MXML: &str = concat!(
    r#"<?xml version="1.0" encoding="utf-8"?>
<mx:Application xmlns:mx="http://www.adobe.com/2006/mxml">
<mx:Label text=""#,
    output_for_hello_world!(),
    r#""/>
</mx:Application>"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mxml,
        extensions: &[FileExtension::Mxml],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mxml,
            syntax: MXML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MXML,
                    [
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
                        (TokenKind::String, r#""http://www.adobe.com/2006/mxml""#),
                        (TokenKind::Keyword, r">"),
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
                )
            },
        }],
    };
