//! Hello-world snippets for `ProgrammingLanguageName::Pascal`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/Pascal_(programming_language)>
//! - <https://www.freepascal.org/docs-html/current/ref/ref.html>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Pascal => "Pascal",
    versions: [Pascal => "Pascal"],
    extensions: [Pas => "pas"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pascal,
        extensions: &[FileExtension::Pas],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pascal,
            syntax: concat!(
                r"program Hello;
begin
    writeln('",
                output_for_hello_world!(),
                r"');
end."
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "program"),
                (TokenKind::Plain, " Hello;\n"),
                (TokenKind::Keyword, "begin"),
                (TokenKind::Plain, "\n    writeln("),
                (
                    TokenKind::String,
                    concat!("'", output_for_hello_world!(), "'"),
                ),
                (TokenKind::Plain, ");\n"),
                (TokenKind::Keyword, "end"),
                (TokenKind::Plain, "."),
            ],
        }],
    };
