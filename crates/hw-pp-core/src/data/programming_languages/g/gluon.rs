//! Hello-world snippet for `ProgrammingLanguageName::Gluon`.
//!
//! Imported from leachim6/hello-world (`g/Gluon.glu` at `a152253`),
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
    language: Gluon => "Gluon",
    versions: [Gluon => "Gluon"],
    extensions: [Glu => "glu"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gluon,
        extensions: &[FileExtension::Glu],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gluon,
            syntax: concat!(
                r#"let io = import! std.io
io.print ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"let io = import! std.io
io.print ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
