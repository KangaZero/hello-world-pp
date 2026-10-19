//! Hello-world snippet for `ProgrammingLanguageName::Mozart`.
//!
//! Imported from leachim6/hello-world (`m/Mozart.oz` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mozart.oz>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Mozart => "Mozart",
    versions: [Mozart => "Mozart"],
    extensions: [Oz => "oz"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mozart,
        extensions: &[FileExtension::Oz],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mozart,
            syntax: concat!(
                r"functor
import
     Application
     System
define
    {System.showInfo '",
                output_for_hello_world!(),
                r"'}
    {Application.exit 0} 
end"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"functor
import
     Application
     System
define
    {System.showInfo '",
                    output_for_hello_world!(),
                    r"'}
    {Application.exit 0} 
end"
                ),
            )],
        }],
    };
