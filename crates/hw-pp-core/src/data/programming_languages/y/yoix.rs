//! Hello-world snippet for `ProgrammingLanguageName::Yoix`.
//!
//! Imported from leachim6/hello-world (`y/Yoix.yx` at `a152253`),
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
    language: Yoix => "Yoix",
    versions: [Yoix => "Yoix"],
    extensions: [Yx => "yx"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Yoix,
        extensions: &[FileExtension::Yx],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Yoix,
            syntax: concat!(
                r#"import yoix.stdio.*;

printf(""#,
                output_for_hello_world!(),
                r#"");"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"import yoix.stdio.*;

printf(""#,
                    output_for_hello_world!(),
                    r#"");"#
                ),
            )],
        }],
    };
