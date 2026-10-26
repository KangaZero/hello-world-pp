//! Hello-world snippet for `ProgrammingLanguageName::Nial`.
//!
//! Imported from leachim6/hello-world (`n/Nial.ndf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/Nial.ndf>
//! - <https://www.wikidata.org/wiki/Q7023815> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Array;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Nial => "Nial",
    versions: [Nial => "Nial"],
    extensions: [Ndf => "ndf"],
}

const NIAL: &str = concat!(
    r"write '",
    output_for_hello_world!(),
    r"';
bye"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nial,
        extensions: &[FileExtension::Ndf],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Array],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nial,
            syntax: NIAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NIAL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"write '",
                            output_for_hello_world!(),
                            r"';
bye"
                        ),
                    )],
                )
            },
        }],
    };
