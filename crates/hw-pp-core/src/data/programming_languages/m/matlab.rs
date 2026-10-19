//! Hello-world snippets for `ProgrammingLanguageName::Matlab`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Matlab => "MATLAB",
    versions: [Matlab => "MATLAB"],
    extensions: [M => "m"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Matlab,
        extensions: &[FileExtension::M],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Matlab,
            syntax: concat!(r"disp('", output_for_hello_world!(), r"')"),
            tokenized_syntax: &[
                (TokenKind::Keyword, "disp"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("'", output_for_hello_world!(), "'"),
                ),
                (TokenKind::Punctuation, ")"),
            ],
        }],
    };
