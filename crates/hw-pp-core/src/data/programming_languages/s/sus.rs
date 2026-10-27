//! Hello-world snippet for `ProgrammingLanguageName::Sus`.
//!
//! Imported from leachim6/hello-world (`s/Sus.sus` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Sus.sus>
//! - <https://esolangs.org/wiki/Sus> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sus => "Sus",
    versions: [Sus => "Sus"],
    extensions: [Sus => "sus"],
}

const SUS: &str = r"-------------------------|>++++|>+++++++++++||>++++++++++++++|>-----------------------------------------------------------------|>++++++++++++++++++++++|<<|>>>+++++++++++++++++|<<<<|>>>>>+++|>----------------------------------------------------------------|";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sus,
        extensions: &[FileExtension::Sus],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sus,
            syntax: SUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SUS,
                    [(
                        TokenKind::Plain,
                        r"-------------------------|>++++|>+++++++++++||>++++++++++++++|>-----------------------------------------------------------------|>++++++++++++++++++++++|<<|>>>+++++++++++++++++|<<<<|>>>>>+++|>----------------------------------------------------------------|",
                    )],
                )
            },
        }],
    };
