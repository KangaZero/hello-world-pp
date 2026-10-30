//! Hello-world snippets for `ProgrammingLanguageName::Python`.
//!
//! Sources:
//! - <https://docs.python.org/3/tutorial/introduction.html>
//! - <https://docs.python.org/2/reference/simple_stmts.html#the-print-statement>
//! - <https://docs.python.org/3/whatsnew/3.0.html>
//! - <https://www.wikidata.org/wiki/Q28865> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Python, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Python => "Python",
    versions: [Python2 => "Python 2", Python3 => "Python 3"],
    extensions: [Py => "py"],
}

const PYTHON2: &str = concat!(r#"print ""#, output_for_hello_world!(), r#"""#);
const PYTHON3: &str = concat!(r#"print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Python,
        extensions: &[FileExtension::Py],
        categories: &[Scripting],
        families: &[Python],
        paradigms: &[Imperative, ObjectOriented, Functional],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Python2,
                syntax: PYTHON2,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        PYTHON2,
                        [
                            (TokenKind::Keyword, "print"),
                            (TokenKind::Plain, " "),
                            (
                                TokenKind::String,
                                concat!("\"", output_for_hello_world!(), "\""),
                            ),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Python3,
                syntax: PYTHON3,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        PYTHON3,
                        [
                            (TokenKind::Function, "print"),
                            (TokenKind::Punctuation, "("),
                            (
                                TokenKind::String,
                                concat!("\"", output_for_hello_world!(), "\""),
                            ),
                            (TokenKind::Punctuation, ")"),
                        ],
                    )
                },
            },
        ],
    };
