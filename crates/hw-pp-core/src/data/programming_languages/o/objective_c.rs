//! Hello-world snippets for `ProgrammingLanguageName::ObjectiveC`.
//!
//! Sources:
//! - <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/Introduction/Introduction.html>
//! - <https://en.wikipedia.org/wiki/Objective-C>
//! - <https://developer.apple.com/documentation/foundation/nslog>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: ObjectiveC => "Objective-C",
    versions: [ObjectiveC2 => "Objective-C 2.0"],
    extensions: [M => "m", Mm => "mm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ObjectiveC,
        extensions: &[FileExtension::M, FileExtension::Mm],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ObjectiveC2,
            syntax: concat!(
                r#"#import <Foundation/Foundation.h>

int main() {
    NSLog(@""#,
                output_for_hello_world!(),
                r#"");
    return 0;
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "#import"),
                (TokenKind::Plain, " "),
                (TokenKind::String, "<Foundation/Foundation.h>"),
                (TokenKind::Plain, "\n\n"),
                (TokenKind::Type, "int"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "main"),
                (TokenKind::Punctuation, "()"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Function, "NSLog"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("@\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ");"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Keyword, "return"),
                (TokenKind::Plain, " "),
                (TokenKind::Number, "0"),
                (TokenKind::Punctuation, ";"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    };
