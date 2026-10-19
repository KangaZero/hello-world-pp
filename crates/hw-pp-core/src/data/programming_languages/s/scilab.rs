//! Hello-world snippet for `ProgrammingLanguageName::Scilab`.
//!
//! Imported from leachim6/hello-world (`s/SCILab.scilab` at `a152253`),
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
    language: Scilab => "SCILab",
    versions: [Scilab => "SCILab"],
    extensions: [Scilab => "scilab"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Scilab,
        extensions: &[FileExtension::Scilab],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Scilab,
            syntax: concat!(r#"disp(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"disp(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
