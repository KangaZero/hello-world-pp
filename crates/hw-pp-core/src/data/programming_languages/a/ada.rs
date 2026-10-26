//! Hello-world snippet for `ProgrammingLanguageName::Ada`.
//!
//! Sources:
//! - <https://learn.adacore.com/courses/intro-to-ada/chapters/imperative_language.html>
//! - <http://www.ada-auth.org/standards/12rm/html/RM-TOC.html>
//! - <https://en.wikipedia.org/wiki/Ada_(programming_language)>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Ada, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ada => "Ada",
    versions: [Ada2012 => "Ada 2012"],
    extensions: [Adb => "adb", Ads => "ads"],
}

const ADA2012: &str = concat!(
    r#"with Ada.Text_IO; use Ada.Text_IO;

procedure Hello is
begin
    Put_Line (""#,
    output_for_hello_world!(),
    r#"");
end Hello;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ada,
        extensions: &[FileExtension::Adb, FileExtension::Ads],
        categories: &[Compiled],
        families: &[Ada],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ada2012,
            syntax: ADA2012,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ADA2012,
                    [
                        (TokenKind::Keyword, "with"),
                        (TokenKind::Plain, " Ada"),
                        (TokenKind::Punctuation, "."),
                        (TokenKind::Plain, "Text_IO"),
                        (TokenKind::Punctuation, ";"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Keyword, "use"),
                        (TokenKind::Plain, " Ada"),
                        (TokenKind::Punctuation, "."),
                        (TokenKind::Plain, "Text_IO"),
                        (TokenKind::Punctuation, ";"),
                        (TokenKind::Plain, "\n\n"),
                        (TokenKind::Keyword, "procedure"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Function, "Hello"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Keyword, "is"),
                        (TokenKind::Plain, "\n"),
                        (TokenKind::Keyword, "begin"),
                        (TokenKind::Plain, "\n    "),
                        (TokenKind::Function, "Put_Line"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Punctuation, "("),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                        (TokenKind::Punctuation, ");"),
                        (TokenKind::Plain, "\n"),
                        (TokenKind::Keyword, "end"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Function, "Hello"),
                        (TokenKind::Punctuation, ";"),
                    ],
                )
            },
        }],
    };
