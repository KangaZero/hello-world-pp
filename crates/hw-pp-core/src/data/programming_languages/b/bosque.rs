//! Hello-world snippet for `ProgrammingLanguageName::Bosque`.
//!
//! Imported from leachim6/hello-world (`b/Bosque.bsq` at `a152253`),
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
    language: Bosque => "Bosque",
    versions: [Bosque => "Bosque"],
    extensions: [Bsq => "bsq"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bosque,
        extensions: &[FileExtension::Bsq],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bosque,
            syntax: concat!(
                r#"namespace NSMain;

entrypoint function main(): String {  
    return ""#,
                output_for_hello_world!(),
                r#"";
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"namespace NSMain;

entrypoint function main(): String {  
    return ""#,
                    output_for_hello_world!(),
                    r#"";
}"#
                ),
            )],
        }],
    };
