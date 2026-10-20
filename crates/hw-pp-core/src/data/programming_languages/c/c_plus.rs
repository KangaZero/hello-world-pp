//! Hello-world snippet for `ProgrammingLanguageName::CPlus`.
//!
//! Imported from leachim6/hello-world (`c/C+` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/C%2B>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: CPlus => "C+",
    versions: [CPlus => "C+"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CPlus,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CPlus,
            syntax: concat!(
                r"THATSCPLUSCODENONOTCPLUSPLUSCPLUS{includ =iostrea; usin namespac st> in mai()|cou == ^^",
                output_for_hello_world!(),
                r"^^ == en>retur -1>?"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"THATSCPLUSCODENONOTCPLUSPLUSCPLUS{includ =iostrea; usin namespac st> in mai()|cou == ^^",
                    output_for_hello_world!(),
                    r"^^ == en>retur -1>?"
                ),
            )],
        }],
    };
