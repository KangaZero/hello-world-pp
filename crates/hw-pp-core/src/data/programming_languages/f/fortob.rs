//! Hello-world snippet for `ProgrammingLanguageName::Fortob`.
//!
//! Imported from leachim6/hello-world (`f/Fortob.fortob` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Fortob.fortob>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Fortob => "Fortob",
    versions: [Fortob => "Fortob"],
    extensions: [Fortob => "fortob"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fortob,
        extensions: &[FileExtension::Fortob],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fortob,
            syntax: concat!(
                r#"@print[$!.readcom.println;$\.push:'\;]\;
print ""#,
                output_for_hello_world!(),
                r#""\;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"@print[$!.readcom.println;$\.push:'\;]\;
print ""#,
                    output_for_hello_world!(),
                    r#""\;"#
                ),
            )],
        }],
    };
