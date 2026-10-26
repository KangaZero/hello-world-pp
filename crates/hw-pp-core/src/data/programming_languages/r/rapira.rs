//! Hello-world snippet for `ProgrammingLanguageName::Rapira`.
//!
//! Imported from leachim6/hello-world (`r/Rapira.rap` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Rapira.rap>
//! - <https://www.wikidata.org/wiki/Q4117140> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Procedural, Structured};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Pascal, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rapira => "Rapira",
    versions: [Rapira => "Rapira"],
    extensions: [Rap => "rap"],
}

const RAPIRA: &str = concat!(r#"output: ""#, output_for_hello_world!(), r#"";"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rapira,
        extensions: &[FileExtension::Rap],
        categories: &[Educational],
        families: &[Pascal],
        paradigms: &[Procedural, Structured],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rapira,
            syntax: RAPIRA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RAPIRA,
                    [(
                        TokenKind::Plain,
                        concat!(r#"output: ""#, output_for_hello_world!(), r#"";"#),
                    )],
                )
            },
        }],
    };
