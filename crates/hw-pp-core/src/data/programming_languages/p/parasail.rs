//! Hello-world snippet for `ProgrammingLanguageName::Parasail`.
//!
//! Imported from leachim6/hello-world (`p/ParaSail.psi` at `a152253`),
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
    language: Parasail => "ParaSail",
    versions: [Parasail => "ParaSail"],
    extensions: [Psi => "psi"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Parasail,
        extensions: &[FileExtension::Psi],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Parasail,
            syntax: concat!(
                r#"func Hello_World(var IO) is
    IO.Println(""#,
                output_for_hello_world!(),
                r#"");
end func Hello_World;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"func Hello_World(var IO) is
    IO.Println(""#,
                    output_for_hello_world!(),
                    r#"");
end func Hello_World;"#
                ),
            )],
        }],
    };
