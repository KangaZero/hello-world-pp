//! Hello-world snippet for `ProgrammingLanguageName::Fortob`.
//!
//! Imported from leachim6/hello-world (`f/Fortob.fortob` at `a152253`),
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
    language: Fortob => "Fortob",
    versions: [Fortob => "Fortob"],
    extensions: [Fortob => "fortob"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fortob,
        extensions: &[FileExtension::Fortob],
        categories: &[],
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
