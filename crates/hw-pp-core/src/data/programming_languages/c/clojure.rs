//! Hello-world snippets for `ProgrammingLanguageName::Clojure`.
//!
//! Sources:
//! - <https://clojure.org/guides/learn/clojure>
//! - <https://clojure.org/releases/devchangelog>
//! - <https://clojure.org/news/2022/03/22/clojure-1-11-0>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Scripting},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Lisp,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Clojure => "Clojure",
    versions: [Clojure111 => "Clojure 1.11"],
    extensions: [Clj => "clj", Cljs => "cljs", Cljc => "cljc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clojure,
        extensions: &[FileExtension::Clj, FileExtension::Cljs, FileExtension::Cljc],
        categories: &[Compiled, Scripting],
        families: &[Lisp],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clojure111,
            syntax: concat!(r#"(println ""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[
                (TokenKind::Plain, "("),
                (TokenKind::Keyword, "println"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Plain, ")"),
            ],
        }],
    };
