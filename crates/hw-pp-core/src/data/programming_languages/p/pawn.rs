//! Hello-world snippet for `ProgrammingLanguageName::Pawn`.
//!
//! Imported from leachim6/hello-world (`p/Pawn.p` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pawn.p>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Pawn => "Pawn",
    versions: [Pawn => "Pawn"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pawn,
        extensions: &[],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pawn,
            syntax: concat!(
                r#"#include <core>
 
main(){
    print(""#,
                output_for_hello_world!(),
                r#"");   
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#include <core>
 
main(){
    print(""#,
                    output_for_hello_world!(),
                    r#"");   
}"#
                ),
            )],
        }],
    };
