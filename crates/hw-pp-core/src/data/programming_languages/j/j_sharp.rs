//! Hello-world snippet for `ProgrammingLanguageName::JSharp`.
//!
//! Imported from leachim6/hello-world (`j/J#.jsl` at `a152253`),
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
    language: JSharp => "J#",
    versions: [JSharp => "J#"],
    extensions: [Jsl => "jsl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::JSharp,
        extensions: &[FileExtension::Jsl],
        categories: &[],
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
