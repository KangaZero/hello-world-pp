//! Hello-world snippet for `ProgrammingLanguageName::Haxe`.
//!
//! Imported from leachim6/hello-world (`h/Haxe.hx` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Haxe.hx>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Haxe => "Haxe",
    versions: [Haxe => "Haxe"],
    extensions: [Hx => "hx"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Haxe,
        extensions: &[FileExtension::Hx],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Haxe,
            syntax: concat!(
                r#"class HelloWorld {
    static function main() {
        trace(""#,
                output_for_hello_world!(),
                r#"");
    }
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"class HelloWorld {
    static function main() {
        trace(""#,
                    output_for_hello_world!(),
                    r#"");
    }
}"#
                ),
            )],
        }],
    };
