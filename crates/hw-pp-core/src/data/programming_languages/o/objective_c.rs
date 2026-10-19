//! Hello-world snippets for `ProgrammingLanguageName::ObjectiveC`.
//!
//! Sources:
//! - <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/Introduction/Introduction.html>
//! - <https://en.wikipedia.org/wiki/Objective-C>
//! - <https://developer.apple.com/documentation/foundation/nslog>
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Objective%20C.m>
//!
//! The `Objective-C (ARC)` version is imported from leachim6/hello-world,
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

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
    versions: [ObjectiveC2 => "Objective-C 2.0", ObjectiveCArc => "Objective-C (ARC)"],
    extensions: [M => "m", Mm => "mm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ObjectiveC,
        extensions: &[FileExtension::M, FileExtension::Mm],
        categories: &[Compiled, Historical],
        versions: &[
            ProgrammingLanguageVersion {
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
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::ObjectiveCArc,
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
            },
        ],
    };
