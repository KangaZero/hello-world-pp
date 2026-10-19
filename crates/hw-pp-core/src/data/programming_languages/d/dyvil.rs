//! Hello-world snippet for `ProgrammingLanguageName::Dyvil`.
//!
//! Imported from leachim6/hello-world (`d/Dyvil.dyv` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Dyvil.dyv>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dyvil => "Dyvil",
    versions: [Dyvil => "Dyvil"],
    extensions: [Dyv => "dyv"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dyvil,
        extensions: &[FileExtension::Dyv],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dyvil,
            syntax: concat!(
                r"class Dyvil
{
	static func main(args: [String]) = print '",
                output_for_hello_world!(),
                r"'
}"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"class Dyvil
{
	static func main(args: [String]) = print '",
                    output_for_hello_world!(),
                    r"'
}"
                ),
            )],
        }],
    };
