//! Hello-world snippet for `ProgrammingLanguageName::Mel`.
//!
//! Imported from leachim6/hello-world (`m/Mel.mel` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mel.mel>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Mel => "Mel",
    versions: [Mel => "Mel"],
    extensions: [Mel => "mel"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mel,
        extensions: &[FileExtension::Mel],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mel,
            syntax: concat!(
                r#"proc helloWorld () {
   print ""#,
                output_for_hello_world!(),
                r#"\n";
}
helloWorld;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"proc helloWorld () {
   print ""#,
                    output_for_hello_world!(),
                    r#"\n";
}
helloWorld;"#
                ),
            )],
        }],
    };
