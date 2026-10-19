//! Hello-world snippet for `ProgrammingLanguageName::Idk`.
//!
//! Imported from leachim6/hello-world (`i/IDK.idk` at `a152253`),
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
    language: Idk => "IDK",
    versions: [Idk => "IDK"],
    extensions: [Idk => "idk"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Idk,
        extensions: &[FileExtension::Idk],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Idk,
            syntax: concat!(
                r"openC
movLoc
2
extract
movVar
",
                output_for_hello_world!(),
                r"
execute
closeC"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"openC
movLoc
2
extract
movVar
",
                    output_for_hello_world!(),
                    r"
execute
closeC"
                ),
            )],
        }],
    };
