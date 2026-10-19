//! Hello-world snippet for `ProgrammingLanguageName::Simula`.
//!
//! Imported from leachim6/hello-world (`s/Simula.sim` at `a152253`),
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
    language: Simula => "Simula",
    versions: [Simula => "Simula"],
    extensions: [Sim => "sim"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Simula,
        extensions: &[FileExtension::Sim],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Simula,
            syntax: concat!(
                r#"Begin
   OutText (""#,
                output_for_hello_world!(),
                r#"");
   Outimage;
End;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"Begin
   OutText (""#,
                    output_for_hello_world!(),
                    r#"");
   Outimage;
End;"#
                ),
            )],
        }],
    };
