//! Hello-world snippet for `ProgrammingLanguageName::Zscript`.
//!
//! Imported from leachim6/hello-world (`z/ZScript` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/z/ZScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Zscript => "ZScript",
    versions: [Zscript => "ZScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zscript,
        extensions: &[],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zscript,
            syntax: concat!(
                r#" version "3.3"
 
 void HelloWorld()
 {
   A_Log(""#,
                output_for_hello_world!(),
                r#"");
 }"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#" version "3.3"
 
 void HelloWorld()
 {
   A_Log(""#,
                    output_for_hello_world!(),
                    r#"");
 }"#
                ),
            )],
        }],
    };
