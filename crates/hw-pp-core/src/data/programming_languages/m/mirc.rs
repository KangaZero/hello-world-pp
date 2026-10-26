//! Hello-world snippet for `ProgrammingLanguageName::Mirc`.
//!
//! Imported from leachim6/hello-world (`m/Mirc.mrc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mirc.mrc>
//! - <https://www.wikidata.org/wiki/Q300841> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{EventDriven, Procedural};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Mirc => "Mirc",
    versions: [Mirc => "Mirc"],
    extensions: [Mrc => "mrc"],
}

const MIRC: &str = concat!(r"echo -a ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mirc,
        extensions: &[FileExtension::Mrc],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Procedural, EventDriven],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mirc,
            syntax: MIRC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MIRC,
                    [(
                        TokenKind::Plain,
                        concat!(r"echo -a ", output_for_hello_world!()),
                    )],
                )
            },
        }],
    };
