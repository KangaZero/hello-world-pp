//! Hello-world snippet for `ProgrammingLanguageName::StackCats`.
//!
//! Imported from m-ender/stackcats (`examples/hello-world.sks` at `7109b1b`),
//! MIT licensed; see `programming_languages/LICENSE-m-ender-stackcats`.
//!
//! Sources:
//! - <https://github.com/m-ender/stackcats/blob/7109b1b6aa3953edcc2c1b40ab88f0bbc6dc7143/examples/hello-world.sks>
//! - <https://esolangs.org/wiki/Stack_Cats>

use crate::data::macros::language_names;
use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

language_names! {
    language: StackCats => "Stack Cats",
    versions: [StackCats => "Stack Cats"],
    extensions: [Sks => "sks"],
}

const STACK_CATS: &str = r"(]<*[[>>]<]^+<[>\]_-]<<<]*_-]]^:[_-:^:+<*]<//[[>>]^:<]:<]]^:[<//]]^:-!]<{>>>[[:_-_-^]<[}]<_!]<_!]<-!*-!^:[:_-_-:[^:]_-:_-:_-:_-_-^:)*-*(:^-_-_:-_:-_:-_[:^]:-_-_:]:^!-*!->[!_>[!_>[{]>[^-_-_:]]<<<}>[!-:^[[\\>]:^[[>:[>:^[<<]]\\>[*>+:^:-_]:^[[-_*[>>>[-_[/<]>+^[>[<<]]*>[)";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::StackCats,
        extensions: &[FileExtension::Sks],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::StackCats,
            syntax: STACK_CATS,
            tokenized_syntax: &const {
                checked_token_lengths(STACK_CATS, [(TokenKind::Plain, STACK_CATS)])
            },
        }],
    };
