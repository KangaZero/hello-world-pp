//! Hello-world snippet for `ProgrammingLanguageName::Fjoelnir`.
//!
//! Imported from leachim6/hello-world (`f/Fjoelnir.fjo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Fjoelnir.fjo>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Fjoelnir => "Fjoelnir",
    versions: [Fjoelnir => "Fjoelnir"],
    extensions: [Fjo => "fjo"],
}

const FJOELNIR: &str = concat!(
    r#""hello" < main
{
    main ->
    stef(;)
    stofn
        skrifastreng(;""#,
    output_for_hello_world!(),
    r#""),
    stofnlok
}
*
"GRUNNUR"
;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fjoelnir,
        extensions: &[FileExtension::Fjo],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fjoelnir,
            syntax: FJOELNIR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FJOELNIR,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#""hello" < main
{
    main ->
    stef(;)
    stofn
        skrifastreng(;""#,
                            output_for_hello_world!(),
                            r#""),
    stofnlok
}
*
"GRUNNUR"
;"#
                        ),
                    )],
                )
            },
        }],
    };
