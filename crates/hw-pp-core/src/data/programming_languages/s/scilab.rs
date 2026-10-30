//! Hello-world snippet for `ProgrammingLanguageName::Scilab`.
//!
//! Imported from leachim6/hello-world (`s/SCILab.scilab` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SCILab.scilab>
//! - <https://www.wikidata.org/wiki/Q828742> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Array;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scientific,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Scilab => "SCILab",
    versions: [Scilab => "SCILab"],
    extensions: [Scilab => "scilab"],
}

const SCILAB: &str = concat!(r#"disp(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Scilab,
        extensions: &[FileExtension::Scilab],
        categories: &[Scientific],
        families: &[],
        paradigms: &[Array],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Scilab,
            syntax: SCILAB,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SCILAB,
                    [
                        (TokenKind::Function, r"disp"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
