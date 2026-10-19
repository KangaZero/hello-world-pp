//! Hello-world snippet for `ProgrammingLanguageName::Haxe`.
//!
//! Imported from leachim6/hello-world (`h/Haxe.hx` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
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
