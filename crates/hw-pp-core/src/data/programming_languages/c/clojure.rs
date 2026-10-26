//! Hello-world snippet for `ProgrammingLanguageName::Clojure`.
//!
//! Sources:
//! - <https://clojure.org/guides/learn/clojure>
//! - <https://clojure.org/releases/devchangelog>
//! - <https://clojure.org/news/2022/03/22/clojure-1-11-0>
//! - <https://www.wikidata.org/wiki/Q51798> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Concurrent, Functional, Logic};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Scripting},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Lisp,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Clojure => "Clojure",
    versions: [Clojure111 => "Clojure 1.11"],
    extensions: [Clj => "clj", Cljs => "cljs", Cljc => "cljc"],
}

const CLOJURE111: &str = concat!(r#"(println ""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clojure,
        extensions: &[FileExtension::Clj, FileExtension::Cljs, FileExtension::Cljc],
        categories: &[Compiled, Scripting],
        families: &[Lisp],
        paradigms: &[Functional, Logic, Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clojure111,
            syntax: CLOJURE111,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CLOJURE111,
                    [
                        (TokenKind::Plain, "("),
                        (TokenKind::Keyword, "println"),
                        (TokenKind::Plain, " "),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                        (TokenKind::Plain, ")"),
                    ],
                )
            },
        }],
    };
