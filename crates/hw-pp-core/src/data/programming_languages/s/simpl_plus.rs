//! Hello-world snippet for `ProgrammingLanguageName::SimplPlus`.
//!
//! Imported from leachim6/hello-world (`s/Simpl+.usp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Simpl%2B.usp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: SimplPlus => "Simpl+",
    versions: [SimplPlus => "Simpl+"],
    extensions: [Usp => "usp"],
}

const SIMPL_PLUS: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in Crestron's SIMPL+
Function Main
{
    Print(""#,
    output_for_hello_world!(),
    r#"\n");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SimplPlus,
        extensions: &[FileExtension::Usp],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SimplPlus,
            syntax: SIMPL_PLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SIMPL_PLUS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r#" in Crestron's SIMPL+
Function Main
{
    Print(""#,
                            output_for_hello_world!(),
                            r#"\n");
}"#
                        ),
                    )],
                )
            },
        }],
    };
