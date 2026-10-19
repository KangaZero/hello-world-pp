//! Hello-world snippet for `ProgrammingLanguageName::Xl`.
//!
//! Imported from leachim6/hello-world (`x/XL.xl` at `a152253`),
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
    language: Xl => "XL",
    versions: [Xl => "XL"],
    extensions: [Xl => "xl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xl,
        extensions: &[FileExtension::Xl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xl,
            syntax: concat!(
                r#"// XL programming language: http://xlr.sourceforge.net
writeln ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"// XL programming language: http://xlr.sourceforge.net
writeln ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
