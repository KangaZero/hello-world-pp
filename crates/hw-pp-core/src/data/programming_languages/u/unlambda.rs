//! Hello-world snippet for `ProgrammingLanguageName::Unlambda`.
//!
//! Imported from leachim6/hello-world (`u/Unlambda.unl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Unlambda => "Unlambda",
    versions: [Unlambda => "Unlambda"],
    extensions: [Unl => "unl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Unlambda,
        extensions: &[FileExtension::Unl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Unlambda,
            syntax: r"`r```````````.H.e.l.l.o. .w.o.r.l.di",
            tokenized_syntax: &[(TokenKind::Plain, r"`r```````````.H.e.l.l.o. .w.o.r.l.di")],
        }],
    };
