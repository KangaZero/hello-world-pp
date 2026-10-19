//! Hello-world snippets for `ProgrammingLanguageName::Html`.
//!
//! Sources:
//! - <https://developer.mozilla.org/en-US/docs/Learn_web_development/Getting_started/Your_first_website/Creating_the_content>
//! - <https://developer.mozilla.org/en-US/docs/Web/HTML>
//! - <https://html.spec.whatwg.org/multipage/>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Html => "HTML",
    versions: [Html5 => "HTML5"],
    extensions: [Html => "html", Htm => "htm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Html,
        extensions: &[FileExtension::Html, FileExtension::Htm],
        categories: &[Markup],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Html5,
            syntax: concat!(
                r"<!DOCTYPE html>
<html>
  <body>
    <p>",
                output_for_hello_world!(),
                r"</p>
  </body>
</html>"
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, "<!"),
                (TokenKind::Plain, "DOCTYPE html"),
                (TokenKind::Punctuation, ">"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "<"),
                (TokenKind::Plain, "html"),
                (TokenKind::Punctuation, ">"),
                (TokenKind::Plain, "\n  "),
                (TokenKind::Punctuation, "<"),
                (TokenKind::Plain, "body"),
                (TokenKind::Punctuation, ">"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Punctuation, "<"),
                (TokenKind::Plain, "p"),
                (TokenKind::Punctuation, ">"),
                (TokenKind::Plain, output_for_hello_world!()),
                (TokenKind::Punctuation, "</"),
                (TokenKind::Plain, "p"),
                (TokenKind::Punctuation, ">"),
                (TokenKind::Plain, "\n  "),
                (TokenKind::Punctuation, "</"),
                (TokenKind::Plain, "body"),
                (TokenKind::Punctuation, ">"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "</"),
                (TokenKind::Plain, "html"),
                (TokenKind::Punctuation, ">"),
            ],
        }],
    };
