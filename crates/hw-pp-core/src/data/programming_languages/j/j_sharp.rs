//! Hello-world snippet for `ProgrammingLanguageName::JSharp`.
//!
//! Imported from leachim6/hello-world (`j/J#.jsl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/J%23.jsl>
//! - <https://www.wikidata.org/wiki/Q1160312> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Imperative, ObjectOriented, Structured,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: JSharp => "J#",
    versions: [JSharp => "J#"],
    extensions: [Jsl => "jsl"],
}

const J_SHARP: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::JSharp,
        extensions: &[FileExtension::Jsl],
        categories: &[Compiled, Historical],
        families: &[],
        paradigms: &[Imperative, Structured, ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::JSharp,
            syntax: J_SHARP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    J_SHARP,
                    [(
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
                )
            },
        }],
    };
