//! Hello-world snippet for `ProgrammingLanguageName::Qcl`.
//!
//! Imported from leachim6/hello-world (`q/QCL.qcl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/q/QCL.qcl>
//! - <https://en.wikipedia.org/wiki/Quantum_programming> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Qcl => "QCL",
    versions: [Qcl => "QCL"],
    extensions: [Qcl => "qcl"],
}

const QCL: &str = concat!(r#"print ""#, output_for_hello_world!(), r#"";"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Qcl,
        extensions: &[FileExtension::Qcl],
        categories: &[],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Qcl,
            syntax: QCL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    QCL,
                    [(
                        TokenKind::Plain,
                        concat!(r#"print ""#, output_for_hello_world!(), r#"";"#),
                    )],
                )
            },
        }],
    };
