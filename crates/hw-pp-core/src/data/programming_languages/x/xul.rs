//! Hello-world snippet for `ProgrammingLanguageName::Xul`.
//!
//! Imported from leachim6/hello-world (`x/XUL.xul` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/XUL.xul>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Xul => "XUL",
    versions: [Xul => "XUL"],
    extensions: [Xul => "xul"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xul,
        extensions: &[FileExtension::Xul],
        categories: &[Markup],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xul,
            syntax: concat!(
                r#"<?xml version="1.0"?>
<window xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul">
    <label value=""#,
                output_for_hello_world!(),
                r#""/>
</window>"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"<"),
                (TokenKind::Plain, r"?xml version="),
                (TokenKind::String, r#""1.0""#),
                (TokenKind::Plain, r"?"),
                (TokenKind::Keyword, r">"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"<window"),
                (TokenKind::Plain, r" xmlns="),
                (
                    TokenKind::String,
                    r#""http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul""#,
                ),
                (TokenKind::Keyword, r">"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"<label"),
                (TokenKind::Plain, r" value="),
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
                (TokenKind::Keyword, r"</window>"),
            ],
        }],
    };
