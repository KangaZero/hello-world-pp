//! Hello-world snippet for `ProgrammingLanguageName::Dyvil`.
//!
//! Imported from leachim6/hello-world (`d/Dyvil.dyv` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Dyvil.dyv>
//! - <https://github.com/dyvil/dyvil> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Dyvil => "Dyvil",
    versions: [Dyvil => "Dyvil"],
    extensions: [Dyv => "dyv"],
}

const DYVIL: &str = concat!(
    r"class Dyvil
{
	static func main(args: [String]) = print '",
    output_for_hello_world!(),
    r"'
}"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dyvil,
        extensions: &[FileExtension::Dyv],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Imperative, ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dyvil,
            syntax: DYVIL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DYVIL,
                    [
                        (TokenKind::Keyword, r"class"),
                        (
                            TokenKind::Plain,
                            r" Dyvil
",
                        ),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"static"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"func"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"args"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Function, r"String"),
                        (TokenKind::Punctuation, r"])"),
                        (TokenKind::Plain, r" = "),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
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
