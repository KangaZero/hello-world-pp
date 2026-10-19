//! Hello-world snippet for `ProgrammingLanguageName::Casl2`.
//!
//! Imported from leachim6/hello-world (`c/CASL2.cas` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Casl2 => "CASL2",
    versions: [Casl2 => "CASL2"],
    extensions: [Cas => "cas"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Casl2,
        extensions: &[FileExtension::Cas],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Casl2,
            syntax: concat!(
                r"MAIN    START
        OUT     MSG,LEN
        RET
MSG     DC      '",
                output_for_hello_world!(),
                r"'
LEN     DC      18
        END"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"MAIN    START
        OUT     MSG,LEN
        RET
MSG     DC      '",
                    output_for_hello_world!(),
                    r"'
LEN     DC      18
        END"
                ),
            )],
        }],
    };
