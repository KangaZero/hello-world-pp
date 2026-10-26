//! Hello-world snippet for `ProgrammingLanguageName::Cebl`.
//!
//! Imported from leachim6/hello-world (`c/CEBL.ceb` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CEBL.ceb>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cebl => "CEBL",
    versions: [Cebl => "CEBL"],
    extensions: [Ceb => "ceb"],
}

const CEBL: &str = concat!(r#"OUT ""#, output_for_hello_world!(), r#"",0"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cebl,
        extensions: &[FileExtension::Ceb],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cebl,
            syntax: CEBL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CEBL,
                    [(
                        TokenKind::Plain,
                        concat!(r#"OUT ""#, output_for_hello_world!(), r#"",0"#),
                    )],
                )
            },
        }],
    };
