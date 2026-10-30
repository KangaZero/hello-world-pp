//! Hello-world snippet for `ProgrammingLanguageName::Cyclone`.
//!
//! Imported from leachim6/hello-world (`c/Cyclone.cyc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cyclone.cyc>
//! - <https://en.wikipedia.org/wiki/C_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Procedural, Structured};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::C, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Cyclone => "Cyclone",
    versions: [Cyclone => "Cyclone"],
    extensions: [Cyc => "cyc"],
}

const CYCLONE: &str = concat!(
    r#"#include <stdio.h>
int main() {
 printf(""#,
    output_for_hello_world!(),
    r#"\n");
 return 0;
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cyclone,
        extensions: &[FileExtension::Cyc],
        categories: &[Compiled],
        families: &[C],
        paradigms: &[Imperative, Procedural, Structured],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cyclone,
            syntax: CYCLONE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CYCLONE,
                    [
                        (TokenKind::Macro, r"#include <stdio.h>"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Type, r"int"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
 ",
                        ),
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
 ",
                        ),
                        (TokenKind::Keyword, r"return"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
