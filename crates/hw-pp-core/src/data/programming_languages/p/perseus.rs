//! Hello-world snippet for `ProgrammingLanguageName::Perseus`.
//!
//! Imported from leachim6/hello-world (`p/Perseus.p++` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Perseus.p%2B%2B>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Perseus => "Perseus",
    versions: [Perseus => "Perseus"],
    extensions: [PPlusPlus => "p++"],
}

const PERSEUS: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Perseus,
        extensions: &[FileExtension::PPlusPlus],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Perseus,
            syntax: PERSEUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PERSEUS,
                    [(
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
                )
            },
        }],
    };
