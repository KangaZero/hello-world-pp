//! Hello-world snippet for `ProgrammingLanguageName::Cocoa`.
//!
//! Imported from the Hello World Collection (`files/Cocoa.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Cocoa.txt>
//! - <http://helloworldcollection.de/#Cocoa>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cocoa => "Cocoa",
    versions: [Cocoa => "Cocoa"],
    extensions: [],
}

const COCOA: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in Cocoa Obj-C (OS X)

#import <Foundation/Foundation.h>

int main (int argc, const char * argv[]) {
    NSAutoreleasePool * pool = [[NSAutoreleasePool alloc] init];

    NSLog(@""#,
    output_for_hello_world!(),
    r#"");
    [pool release];
    return 0;
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cocoa,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cocoa,
            syntax: COCOA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COCOA,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r#" in Cocoa Obj-C (OS X)

#import <Foundation/Foundation.h>

int main (int argc, const char * argv[]) {
    NSAutoreleasePool * pool = [[NSAutoreleasePool alloc] init];

    NSLog(@""#,
                            output_for_hello_world!(),
                            r#"");
    [pool release];
    return 0;
}"#
                        ),
                    )],
                )
            },
        }],
    };
