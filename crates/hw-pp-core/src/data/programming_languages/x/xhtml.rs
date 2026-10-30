//! Hello-world snippet for `ProgrammingLanguageName::Xhtml`.
//!
//! Imported from leachim6/hello-world (`x/XHTML.xhtml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/XHTML.xhtml>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Xhtml => "XHTML",
    versions: [Xhtml => "XHTML"],
    extensions: [Xhtml => "xhtml"],
}

const XHTML: &str = concat!(
    r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN"
  "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html xmlns="http://www.w3.org/TR/xhtml1" xml:lang="en" lang="en">
  <head>
    <title>"#,
    output_for_hello_world!(),
    r#"</title>
    <meta charset="UTF-8"/>
  </head>
  <body>
    <p>"#,
    output_for_hello_world!(),
    r"</p>
  </body>
</html>"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xhtml,
        extensions: &[FileExtension::Xhtml],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xhtml,
            syntax: XHTML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    XHTML,
                    [
                        (TokenKind::Punctuation, r"<!"),
                        (TokenKind::Plain, r"DOCTYPE html PUBLIC "),
                        (
                            TokenKind::String,
                            r#""-//W3C//DTD XHTML 1.0 Transitional//EN""#,
                        ),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (
                            TokenKind::String,
                            r#""http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd""#,
                        ),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"html xmlns"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""http://www.w3.org/TR/xhtml1""#),
                        (TokenKind::Plain, r" xml:lang"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""en""#),
                        (TokenKind::Plain, r" lang"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""en""#),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"head"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"title"),
                        (TokenKind::Punctuation, r">"),
                        (TokenKind::Plain, output_for_hello_world!()),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"title"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"meta charset"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""UTF-8""#),
                        (TokenKind::Punctuation, r"/>"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"head"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"body"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"p"),
                        (TokenKind::Punctuation, r">"),
                        (TokenKind::Plain, output_for_hello_world!()),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"p"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"body"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"html"),
                        (TokenKind::Punctuation, r">"),
                    ],
                )
            },
        }],
    };
