//! Hello-world snippet for `ProgrammingLanguageName::Zsharp`.
//!
//! Imported from leachim6/hello-world (`z/ZSharp.zs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/z/ZSharp.zs>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Zsharp => "ZSharp",
    versions: [Zsharp => "ZSharp"],
    extensions: [],
}

const ZSHARP: &str = concat!(
    r#"func Main()
{
	Printl(""#,
    output_for_hello_world!(),
    r#"")
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zsharp,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zsharp,
            syntax: ZSHARP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ZSHARP,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"func Main()
{
	Printl(""#,
                            output_for_hello_world!(),
                            r#"")
}"#
                        ),
                    )],
                )
            },
        }],
    };
