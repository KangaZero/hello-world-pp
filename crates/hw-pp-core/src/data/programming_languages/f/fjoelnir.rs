//! Hello-world snippet for `ProgrammingLanguageName::Fjoelnir`.
//!
//! Imported from leachim6/hello-world (`f/Fjoelnir.fjo` at `a152253`),
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
    language: Fjoelnir => "Fjoelnir",
    versions: [Fjoelnir => "Fjoelnir"],
    extensions: [Fjo => "fjo"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fjoelnir,
        extensions: &[FileExtension::Fjo],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fjoelnir,
            syntax: concat!(
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
            tokenized_syntax: &[(
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
        }],
    };
