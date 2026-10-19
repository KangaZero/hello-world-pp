//! Hello-world snippet for `ProgrammingLanguageName::Mathematica`.
//!
//! Imported from leachim6/hello-world (`m/Mathematica.nb` at `a152253`),
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
    language: Mathematica => "Mathematica",
    versions: [Mathematica => "Mathematica"],
    extensions: [Nb => "nb"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mathematica,
        extensions: &[FileExtension::Nb],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mathematica,
            syntax: concat!(r#"Print[""#, output_for_hello_world!(), r#""]"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"Print[""#, output_for_hello_world!(), r#""]"#),
            )],
        }],
    };
