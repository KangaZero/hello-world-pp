//! Hello-world snippet for `ProgrammingLanguageName::Vala`.
//!
//! Imported from leachim6/hello-world (`v/Vala.vala` at `a152253`),
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
    language: Vala => "Vala",
    versions: [Vala => "Vala"],
    extensions: [Vala => "vala"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vala,
        extensions: &[FileExtension::Vala],
        categories: &[],
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
