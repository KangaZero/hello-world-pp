//! Hello-world snippet for `ProgrammingLanguageName::Cool`.
//!
//! Imported from leachim6/hello-world (`c/Cool.cl` at `a152253`),
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
    language: Cool => "Cool",
    versions: [Cool => "Cool"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cool,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cool,
            syntax: concat!(
                r#"class Main inherits IO {
   main(): Object {
	out_string(""#,
                output_for_hello_world!(),
                r#".\n")
   };
};"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"class Main inherits IO {
   main(): Object {
	out_string(""#,
                    output_for_hello_world!(),
                    r#".\n")
   };
};"#
                ),
            )],
        }],
    };
