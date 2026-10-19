//! Hello-world snippet for `ProgrammingLanguageName::Vala`.
//!
//! Imported from leachim6/hello-world (`v/Vala.vala` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/Vala.vala>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Vala => "Vala",
    versions: [Vala => "Vala"],
    extensions: [Vala => "vala"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vala,
        extensions: &[FileExtension::Vala],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vala,
            syntax: concat!(
                r#"static void main (string[] args)
{
	stdout.printf (""#,
                output_for_hello_world!(),
                r#"\n");
}
"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"static void main (string[] args)
{
	stdout.printf (""#,
                    output_for_hello_world!(),
                    r#"\n");
}
"#
                ),
            )],
        }],
    };
