//! Hello-world snippet for `ProgrammingLanguageName::Jade`.
//!
//! Imported from leachim6/hello-world (`j/Jade.jade` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/Jade.jade>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Jade => "Jade",
    versions: [Jade => "Jade"],
    extensions: [Jade => "jade"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jade,
        extensions: &[FileExtension::Jade],
        categories: &[Markup],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jade,
            syntax: concat!(
                r#"helloWorld();

begin
   write ""#,
                output_for_hello_world!(),
                r#"";
end;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"helloWorld();

begin
   write ""#,
                    output_for_hello_world!(),
                    r#"";
end;"#
                ),
            )],
        }],
    };
