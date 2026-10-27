//! Hello-world snippet for `ProgrammingLanguageName::Factors`.
//!
//! Imported from leachim6/hello-world (`f/Factors.fact` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Factors.fact>
//! - <https://esolangs.org/wiki/Factor> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{CellBased, Imperative};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Factors => "Factors",
    versions: [Factors => "Factors"],
    extensions: [Fact => "fact"],
}

const FACTORS: &str = r"1240479476546339573634662660744997483390159825927257577642211238764859751532647897788588745690462747
2954259026932147401271063216759368974236989467343884314431173679900151962232540368081876543214803741
27355669151693462637517323355034011282348481257152244997524115792512161";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Factors,
        extensions: &[FileExtension::Fact],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Factors,
            syntax: FACTORS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FACTORS,
                    [(
                        TokenKind::Plain,
                        r"1240479476546339573634662660744997483390159825927257577642211238764859751532647897788588745690462747
2954259026932147401271063216759368974236989467343884314431173679900151962232540368081876543214803741
27355669151693462637517323355034011282348481257152244997524115792512161",
                    )],
                )
            },
        }],
    };
