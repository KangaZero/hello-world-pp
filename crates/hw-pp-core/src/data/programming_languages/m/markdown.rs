//! Hello-world snippets for `ProgrammingLanguageName::Markdown`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Markdown => "Markdown",
    versions: [CommonMark => "CommonMark"],
    extensions: [Md => "md", Markdown => "markdown"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Markdown,
        extensions: &[FileExtension::Md, FileExtension::Markdown],
        categories: &[Markup],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CommonMark,
            syntax: concat!(r"# ", output_for_hello_world!()),
            tokenized_syntax: &[
                (TokenKind::Punctuation, "#"),
                (TokenKind::Plain, concat!(" ", output_for_hello_world!())),
            ],
        }],
    };
