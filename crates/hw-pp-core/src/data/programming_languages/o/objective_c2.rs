//! Hello-world snippet for `ProgrammingLanguageName::ObjectiveC2`.
//!
//! Imported from leachim6/hello-world (`o/Objective C.m` at `a152253`),
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
    language: ObjectiveC2 => "Objective C",
    versions: [ObjectiveC22 => "Objective C"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ObjectiveC2,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ObjectiveC22,
            syntax: concat!(
                r#"/*
 Build on OS X: 
 clang -framework Foundation -fobjc-arc objc.m -o objc
 
 Build on Linux with GNUstep:
 clang `gnustep-config --objc-flags` `gnustep-config --base-libs` -fobjc-nonfragile-abi -fobjc-arc objc.m -o objc
 */

#import <Foundation/Foundation.h>

int main(void)
{
    NSLog(@""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    r"/*
 Build on OS X: 
 clang -framework Foundation -fobjc-arc objc.m -o objc
 
 Build on Linux with GNUstep:
 clang `gnustep-config --objc-flags` `gnustep-config --base-libs` -fobjc-nonfragile-abi -fobjc-arc objc.m -o objc
 */",
                ),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"#import"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r"<Foundation/Foundation.h>"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Type, r"int"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"main"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Type, r"void"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"NSLog"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"@""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };
