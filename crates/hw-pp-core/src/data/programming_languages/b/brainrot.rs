//! Hello-world snippet for `ProgrammingLanguageName::Brainrot`.
//!
//! Imported from leachim6/hello-world (`b/Brainrot.brainrot` at `a152253`),
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
    language: Brainrot => "Brainrot",
    versions: [Brainrot => "Brainrot"],
    extensions: [Brainrot => "brainrot"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brainrot,
        extensions: &[FileExtension::Brainrot],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brainrot,
            syntax: concat!(
                r#"skibidi main {
    yapping(""#,
                output_for_hello_world!(),
                r#"");
    bussin 0;
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"skibidi main {
    yapping(""#,
                    output_for_hello_world!(),
                    r#"");
    bussin 0;
}"#
                ),
            )],
        }],
    };
