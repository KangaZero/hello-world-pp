//! Hello-world snippet for `ProgrammingLanguageName::DU266dU266d`.
//!
//! Imported from leachim6/hello-world (`d/D♭♭` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/D%E2%99%AD%E2%99%AD>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: DU266dU266d => "D♭♭",
    versions: [DU266dU266d => "D♭♭"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::DU266dU266d,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::DU266dU266d,
            syntax: concat!(
                r#"#include <stdio.h>
int main() {
   printf(""#,
                output_for_hello_world!(),
                r#"");
   return 0;
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#include <stdio.h>
int main() {
   printf(""#,
                    output_for_hello_world!(),
                    r#"");
   return 0;
}"#
                ),
            )],
        }],
    };
