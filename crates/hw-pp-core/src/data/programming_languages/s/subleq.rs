//! Hello-world snippet for `ProgrammingLanguageName::Subleq`.
//!
//! Imported from leachim6/hello-world (`s/SubleQ.sq` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SubleQ.sq>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Subleq => "SubleQ",
    versions: [Subleq => "SubleQ"],
    extensions: [Sq => "sq"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Subleq,
        extensions: &[FileExtension::Sq],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Subleq,
            syntax: concat!(
                r#"loop:       hello (-1)
            minusOne loop
            minusOne checkEnd+1
checkEnd:   Z hello (-1) 
            Z Z loop
				
. minusOne: -1
. hello: ""#,
                output_for_hello_world!(),
                r#"\n" Z: 0"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"loop:       hello (-1)
            minusOne loop
            minusOne checkEnd+1
checkEnd:   Z hello (-1) 
            Z Z loop
				
. minusOne: -1
. hello: ""#,
                    output_for_hello_world!(),
                    r#"\n" Z: 0"#
                ),
            )],
        }],
    };
