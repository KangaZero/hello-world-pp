//! Hello-world snippet for `ProgrammingLanguageName::Pikalang`.
//!
//! Imported from leachim6/hello-world (`p/PikaLang.pokeball` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PikaLang.pokeball>
//! - <https://esolangs.org/wiki/Pikalang> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Pikalang => "PikaLang",
    versions: [Pikalang => "PikaLang"],
    extensions: [Pokeball => "pokeball"],
}

const PIKALANG: &str = r"pi pi pi pi pi pi pi pi pi pi pika pipi pi pi pi pi pi pi pi pipi pi pi
pi pi pi pi pi pi pi pi pipi pi pi pi pipi pi pichu pichu pichu pichu ka
chu pipi pi pi pikachu pipi pi pikachu pi pi pi pi pi pi pi pikachu
pikachu pi pi pi pikachu pipi pi pi pikachu pichu pichu pi pi pi pi pi
pi pi pi pi pi pi pi pi pi pi pikachu pipi pikachu pi pi pi pikachu ka
ka ka ka ka ka pikachu ka ka ka ka ka ka ka ka pikachu pipi pi pikachu
pipi pikachu ";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pikalang,
        extensions: &[FileExtension::Pokeball],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pikalang,
            syntax: PIKALANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PIKALANG,
                    [(
                        TokenKind::Plain,
                        r"pi pi pi pi pi pi pi pi pi pi pika pipi pi pi pi pi pi pi pi pipi pi pi
pi pi pi pi pi pi pi pi pipi pi pi pi pipi pi pichu pichu pichu pichu ka
chu pipi pi pi pikachu pipi pi pikachu pi pi pi pi pi pi pi pikachu
pikachu pi pi pi pikachu pipi pi pi pikachu pichu pichu pi pi pi pi pi
pi pi pi pi pi pi pi pi pi pi pikachu pipi pikachu pi pi pi pikachu ka
ka ka ka ka ka pikachu ka ka ka ka ka ka ka ka pikachu pipi pi pikachu
pipi pikachu ",
                    )],
                )
            },
        }],
    };
