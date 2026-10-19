//! Hello-world snippet for `ProgrammingLanguageName::Beef`.
//!
//! Imported from leachim6/hello-world (`b/Beef.bf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Beef.bf>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Esolang],
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
