//! Hello-world snippet for `ProgrammingLanguageName::Nemerle`.
//!
//! Imported from leachim6/hello-world (`n/Nemerle.n` at `a152253`),
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
    language: Nemerle => "Nemerle",
    versions: [Nemerle => "Nemerle"],
    extensions: [N => "n"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nemerle,
        extensions: &[FileExtension::N],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nemerle,
            syntax: concat!(
                r#"class Hello
{
    static Main () : void
    {
        System.Console.WriteLine (""#,
                output_for_hello_world!(),
                r#"");
    }
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"class Hello
{
    static Main () : void
    {
        System.Console.WriteLine (""#,
                    output_for_hello_world!(),
                    r#"");
    }
}"#
                ),
            )],
        }],
    };
