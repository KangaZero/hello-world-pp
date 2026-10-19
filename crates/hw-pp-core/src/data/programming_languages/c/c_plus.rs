//! Hello-world snippet for `ProgrammingLanguageName::CPlus`.
//!
//! Imported from leachim6/hello-world (`c/C+` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
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
