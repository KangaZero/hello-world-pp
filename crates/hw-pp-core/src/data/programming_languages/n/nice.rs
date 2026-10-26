//! Hello-world snippet for `ProgrammingLanguageName::Nice`.
//!
//! Imported from leachim6/hello-world (`n/Nice.nice` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/Nice.nice>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Nice => "Nice",
    versions: [Nice => "Nice"],
    extensions: [Nice => "nice"],
}

const NICE: &str = concat!(
    r#"void main(String[] args){
   println(""#,
    output_for_hello_world!(),
    r#"");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nice,
        extensions: &[FileExtension::Nice],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nice,
            syntax: NICE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NICE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"void main(String[] args){
   println(""#,
                            output_for_hello_world!(),
                            r#"");
}"#
                        ),
                    )],
                )
            },
        }],
    };
