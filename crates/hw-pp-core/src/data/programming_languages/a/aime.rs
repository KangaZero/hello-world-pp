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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Aime => "Aime",
    versions: [Aime => "Aime"],
    extensions: [Aime => "aime"],
}

const AIME: &str = concat!(
    r#"integer
main(void)
{
    o_text(""#,
    output_for_hello_world!(),
    r#"\n");
 
    return 0;
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Aime,
        extensions: &[FileExtension::Aime],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Aime,
            syntax: AIME,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AIME,
                    [(
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
                )
            },
        }],
    };
