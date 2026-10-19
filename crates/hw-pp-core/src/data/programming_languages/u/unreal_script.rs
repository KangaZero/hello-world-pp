//! Hello-world snippet for `ProgrammingLanguageName::UnrealScript`.
//!
//! Imported from leachim6/hello-world (`u/Unreal Script.uc` at `a152253`),
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
    language: UnrealScript => "Unreal Script",
    versions: [UnrealScript => "Unreal Script"],
    extensions: [Uc => "uc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::UnrealScript,
        extensions: &[FileExtension::Uc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::UnrealScript,
            syntax: concat!(
                r#"class HelloWorld extends Mutator;

function PostBeginPlay()
{
  Super.PostBeginPlay();
  Log(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"class HelloWorld extends Mutator;

function PostBeginPlay()
{
  Super.PostBeginPlay();
  Log(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
            )],
        }],
    };
