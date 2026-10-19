//! Hello-world snippet for `ProgrammingLanguageName::Cyclone`.
//!
//! Imported from leachim6/hello-world (`c/Cyclone.cyc` at `a152253`),
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
    language: Cyclone => "Cyclone",
    versions: [Cyclone => "Cyclone"],
    extensions: [Cyc => "cyc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cyclone,
        extensions: &[FileExtension::Cyc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cyclone,
            syntax: concat!(
                r#"#include <stdio.h>
int main() {
 printf(""#,
                output_for_hello_world!(),
                r#"\n");
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
                    r#"\n");
 return 0;
}"#
                ),
            )],
        }],
    };
