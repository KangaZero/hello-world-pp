//! Hello-world snippet for `ProgrammingLanguageName::Aime`.
//!
//! Imported from leachim6/hello-world (`a/Aime.aime` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Aime.aime>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Aime => "Aime",
    versions: [Aime => "Aime"],
    extensions: [Aime => "aime"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Aime,
        extensions: &[FileExtension::Aime],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Aime,
            syntax: concat!(
                r#"integer
main(void)
{
    o_text(""#,
                output_for_hello_world!(),
                r#"\n");
 
    return 0;
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"integer
main(void)
{
    o_text(""#,
                    output_for_hello_world!(),
                    r#"\n");
 
    return 0;
}"#
                ),
            )],
        }],
    };
