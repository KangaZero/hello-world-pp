//! Hello-world snippet for `ProgrammingLanguageName::Qugord`.
//!
//! Imported from leachim6/hello-world (`q/Qugord.qugord` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/q/Qugord.qugord>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Qugord => "Qugord",
    versions: [Qugord => "Qugord"],
    extensions: [Qugord => "qugord"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Qugord,
        extensions: &[FileExtension::Qugord],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Qugord,
            syntax: concat!(
                r#"~group hello from %2 to %14 also affect hello into ""#,
                output_for_hello_world!(),
                r#"" also each hello into 0 task give 0 to 1 also at 1 join 1 and %1
~publish also split hello"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"~group hello from %2 to %14 also affect hello into ""#,
                    output_for_hello_world!(),
                    r#"" also each hello into 0 task give 0 to 1 also at 1 join 1 and %1
~publish also split hello"#
                ),
            )],
        }],
    };
