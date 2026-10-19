//! Hello-world snippet for `ProgrammingLanguageName::Cuda`.
//!
//! Imported from leachim6/hello-world (`c/Cuda.cu` at `a152253`),
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
    language: Cuda => "Cuda",
    versions: [Cuda => "Cuda"],
    extensions: [Cu => "cu"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cuda,
        extensions: &[FileExtension::Cu],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cuda,
            syntax: concat!(
                r#"#include <stdio.h>

__global__ void hello_world(){
    printf(""#,
                output_for_hello_world!(),
                r#"\n");
}

int main() {
    hello_world<<<1,1>>>();
    return 0;
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#include <stdio.h>

__global__ void hello_world(){
    printf(""#,
                    output_for_hello_world!(),
                    r#"\n");
}

int main() {
    hello_world<<<1,1>>>();
    return 0;
}"#
                ),
            )],
        }],
    };
