//! Hello-world snippet for `ProgrammingLanguageName::Bosque`.
//!
//! Imported from leachim6/hello-world (`b/Bosque.bsq` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Bosque.bsq>
//! - <https://www.wikidata.org/wiki/Q63243349> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Bosque => "Bosque",
    versions: [Bosque => "Bosque"],
    extensions: [Bsq => "bsq"],
}

const BOSQUE: &str = concat!(
    r#"namespace NSMain;

entrypoint function main(): String {  
    return ""#,
    output_for_hello_world!(),
    r#"";
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bosque,
        extensions: &[FileExtension::Bsq],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bosque,
            syntax: BOSQUE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BOSQUE,
                    [
                        (TokenKind::Keyword, r"namespace"),
                        (TokenKind::Plain, r" NSMain"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

entrypoint ",
                        ),
                        (TokenKind::Keyword, r"function"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"():"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"String"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"  
    ",
                        ),
                        (TokenKind::Keyword, r"return"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
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
