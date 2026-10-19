//! Hello-world snippet for `ProgrammingLanguageName::SimplPlus`.
//!
//! Imported from leachim6/hello-world (`s/Simpl+.usp` at `a152253`),
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
    language: SimplPlus => "Simpl+",
    versions: [SimplPlus => "Simpl+"],
    extensions: [Usp => "usp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SimplPlus,
        extensions: &[FileExtension::Usp],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SimplPlus,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r#" in Crestron's SIMPL+
Function Main
{
    Print(""#,
                output_for_hello_world!(),
                r#"\n");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in Crestron's SIMPL+
Function Main
{
    Print(""#,
                    output_for_hello_world!(),
                    r#"\n");
}"#
                ),
            )],
        }],
    };
