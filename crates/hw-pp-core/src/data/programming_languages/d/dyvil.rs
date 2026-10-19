//! Hello-world snippet for `ProgrammingLanguageName::Dyvil`.
//!
//! Imported from leachim6/hello-world (`d/Dyvil.dyv` at `a152253`),
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
    language: Dyvil => "Dyvil",
    versions: [Dyvil => "Dyvil"],
    extensions: [Dyv => "dyv"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dyvil,
        extensions: &[FileExtension::Dyv],
        categories: &[],
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
