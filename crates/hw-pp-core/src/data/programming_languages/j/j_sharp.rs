//! Hello-world snippet for `ProgrammingLanguageName::JSharp`.
//!
//! Imported from leachim6/hello-world (`j/J#.jsl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/J%23.jsl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: JSharp => "J#",
    versions: [JSharp => "J#"],
    extensions: [Jsl => "jsl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::JSharp,
        extensions: &[FileExtension::Jsl],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::JSharp,
            syntax: concat!(
                r#"package HelloWorld;

public class HelloWorld
{
	public static void main(String[] args)
	{
		System.Console.Write(""#,
                output_for_hello_world!(),
                r#"");
	}
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"package HelloWorld;

public class HelloWorld
{
	public static void main(String[] args)
	{
		System.Console.Write(""#,
                    output_for_hello_world!(),
                    r#"");
	}
}"#
                ),
            )],
        }],
    };
