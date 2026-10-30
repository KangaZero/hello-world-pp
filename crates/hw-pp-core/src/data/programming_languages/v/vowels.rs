//! Hello-world snippet for `ProgrammingLanguageName::Vowels`.
//!
//! Imported from leachim6/hello-world (`v/Vowels.vw` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/Vowels.vw>
//! - <https://esolangs.org/wiki/Vowels_(2017)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Vowels => "Vowels",
    versions: [Vowels => "Vowels"],
    extensions: [Vw => "vw"],
}

const VOWELS: &str = r"ue
yeouo
euea
euai
euai
euoa
eaiu
eaao
eioo
euoa
euou
euai
euee
eaai
eeoe
eeee
y";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vowels,
        extensions: &[FileExtension::Vw],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vowels,
            syntax: VOWELS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VOWELS,
                    [(
                        TokenKind::Plain,
                        r"ue
yeouo
euea
euai
euai
euoa
eaiu
eaao
eioo
euoa
euou
euai
euee
eaai
eeoe
eeee
y",
                    )],
                )
            },
        }],
    };
