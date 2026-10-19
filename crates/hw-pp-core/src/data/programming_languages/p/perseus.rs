//! Hello-world snippet for `ProgrammingLanguageName::Perseus`.
//!
//! Imported from leachim6/hello-world (`p/Perseus.p++` at `a152253`),
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
    language: Perseus => "Perseus",
    versions: [Perseus => "Perseus"],
    extensions: [PPlusPlus => "p++"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Perseus,
        extensions: &[FileExtension::PPlusPlus],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Perseus,
            syntax: concat!(
                r#"link CUI entry main

include "Windows.inc"
include "Console.inc"

func main()
{
	Console.Init("Console")
	Console.Write(""#,
                output_for_hello_world!(),
                r#"")
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"link CUI entry main

include "Windows.inc"
include "Console.inc"

func main()
{
	Console.Init("Console")
	Console.Write(""#,
                    output_for_hello_world!(),
                    r#"")
}"#
                ),
            )],
        }],
    };
