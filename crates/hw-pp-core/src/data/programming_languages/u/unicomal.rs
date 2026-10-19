//! Hello-world snippet for `ProgrammingLanguageName::Unicomal`.
//!
//! Imported from leachim6/hello-world (`u/Unicomal.lst` at `a152253`),
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
    language: Unicomal => "Unicomal",
    versions: [Unicomal => "Unicomal"],
    extensions: [Lst => "lst"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Unicomal,
        extensions: &[FileExtension::Lst],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Unicomal,
            syntax: concat!(r#"0100 PRINT ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"0100 PRINT ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
