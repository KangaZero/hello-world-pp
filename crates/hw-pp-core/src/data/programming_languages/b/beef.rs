//! Hello-world snippet for `ProgrammingLanguageName::Beef`.
//!
//! Imported from leachim6/hello-world (`b/Beef.bf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Beef => "Beef",
    versions: [Beef => "Beef"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Beef,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Beef,
            syntax: concat!(
                r#"using System;

namespace HelloWorld
{
    class Program
    {
        static void Main()
        {
            Console.WriteLine(""#,
                output_for_hello_world!(),
                r#"");
        }
    }
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"using System;

namespace HelloWorld
{
    class Program
    {
        static void Main()
        {
            Console.WriteLine(""#,
                    output_for_hello_world!(),
                    r#"");
        }
    }
}"#
                ),
            )],
        }],
    };
