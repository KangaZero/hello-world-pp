//! Hello-world snippet for `ProgrammingLanguageName::Morloc`.
//!
//! Imported from leachim6/hello-world (`m/Morloc.loc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Morloc.loc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Morloc => "Morloc",
    versions: [Morloc => "Morloc"],
    extensions: [Loc => "loc"],
}

const MORLOC: &str = concat!(
    r#"export hello
hello = ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Morloc,
        extensions: &[FileExtension::Loc],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Morloc,
            syntax: MORLOC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MORLOC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"export hello
hello = ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
