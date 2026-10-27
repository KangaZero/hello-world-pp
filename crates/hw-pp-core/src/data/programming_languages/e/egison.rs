//! Hello-world snippet for `ProgrammingLanguageName::Egison`.
//!
//! Imported from leachim6/hello-world (`e/Egison.egi` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Egison.egi>
//! - <https://github.com/egison/egison> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Egison => "Egison",
    versions: [Egison => "Egison"],
    extensions: [Egi => "egi"],
}

const EGISON: &str = concat!(
    r#"def main args := write ""#,
    output_for_hello_world!(),
    r#"\n""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Egison,
        extensions: &[FileExtension::Egi],
        categories: &[],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Egison,
            syntax: EGISON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EGISON,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"def main args := write ""#,
                            output_for_hello_world!(),
                            r#"\n""#
                        ),
                    )],
                )
            },
        }],
    };
