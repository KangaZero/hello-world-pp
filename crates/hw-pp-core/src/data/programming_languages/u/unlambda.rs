//! Hello-world snippet for `ProgrammingLanguageName::Unlambda`.
//!
//! Imported from leachim6/hello-world (`u/Unlambda.unl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/u/Unlambda.unl>
//! - <https://www.wikidata.org/wiki/Q670180> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Unlambda => "Unlambda",
    versions: [Unlambda => "Unlambda"],
    extensions: [Unl => "unl"],
}

const UNLAMBDA: &str = r"`r```````````.H.e.l.l.o. .w.o.r.l.di";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Unlambda,
        extensions: &[FileExtension::Unl],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Unlambda,
            syntax: UNLAMBDA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    UNLAMBDA,
                    [(TokenKind::Plain, r"`r```````````.H.e.l.l.o. .w.o.r.l.di")],
                )
            },
        }],
    };
