//! Hello-world snippet for `ProgrammingLanguageName::Qugord`.
//!
//! Imported from leachim6/hello-world (`q/Qugord.qugord` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/q/Qugord.qugord>
//! - <https://esolangs.org/wiki/Doorspace> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, ObjectOriented};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Qugord => "Qugord",
    versions: [Qugord => "Qugord"],
    extensions: [Qugord => "qugord"],
}

const QUGORD: &str = concat!(
    r#"~group hello from %2 to %14 also affect hello into ""#,
    output_for_hello_world!(),
    r#"" also each hello into 0 task give 0 to 1 also at 1 join 1 and %1
~publish also split hello"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Qugord,
        extensions: &[FileExtension::Qugord],
        categories: &[],
        families: &[],
        paradigms: &[ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Qugord,
            syntax: QUGORD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    QUGORD,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"~group hello from %2 to %14 also affect hello into ""#,
                            output_for_hello_world!(),
                            r#"" also each hello into 0 task give 0 to 1 also at 1 join 1 and %1
~publish also split hello"#
                        ),
                    )],
                )
            },
        }],
    };
