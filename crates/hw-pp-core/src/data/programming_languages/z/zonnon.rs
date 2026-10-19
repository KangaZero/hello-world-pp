//! Hello-world snippet for `ProgrammingLanguageName::Zonnon`.
//!
//! Imported from leachim6/hello-world (`z/Zonnon.znn` at `a152253`),
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
    language: Zonnon => "Zonnon",
    versions: [Zonnon => "Zonnon"],
    extensions: [Znn => "znn"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zonnon,
        extensions: &[FileExtension::Znn],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zonnon,
            syntax: concat!(
                r#"module HelloWorld;
begin
    writeln(""#,
                output_for_hello_world!(),
                r#"");
end HelloWorld.    "#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"module HelloWorld;
begin
    writeln(""#,
                    output_for_hello_world!(),
                    r#"");
end HelloWorld.    "#
                ),
            )],
        }],
    };
