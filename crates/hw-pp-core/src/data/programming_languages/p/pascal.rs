//! Hello-world snippets for `ProgrammingLanguageName::Pascal`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/Pascal_(programming_language)>
//! - <https://www.freepascal.org/docs-html/current/ref/ref.html>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Pascal%26nbsp%3B%28Windows%29>
//! - <http://helloworldcollection.de/#Pascal%C2%A0(Windows)>
//! - <https://www.wikidata.org/wiki/Q81571> (paradigms)
//!
//! The `Pascal (Windows)` version is imported from the Hello World Collection, MIT licensed; see
//! `programming_languages/LICENSE-helloworldcollection`.

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Structured};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Pascal, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Pascal => "Pascal",
    versions: [Pascal => "Pascal", PascalWindows => "Pascal (Windows)"],
    extensions: [Pas => "pas"],
}

const PASCAL: &str = concat!(
    r"program Hello;
begin
    writeln('",
    output_for_hello_world!(),
    r"');
end."
);
const PASCAL_WINDOWS: &str = concat!(
    r"{ ",
    output_for_hello_world!(),
    r" in Borland Pascal 7 for MS-Windows}

PROGRAM HelloWorld;

USES
  WinCRT;

BEGIN
  InitWinCRT;
  WriteLn('",
    output_for_hello_world!(),
    r"');
  ReadLn;
  DoneWinCRT;
END."
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pascal,
        extensions: &[FileExtension::Pas],
        categories: &[Compiled],
        families: &[Pascal],
        paradigms: &[Imperative, Structured],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Pascal,
                syntax: PASCAL,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        PASCAL,
                        [
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
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::PascalWindows,
                syntax: PASCAL_WINDOWS,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        PASCAL_WINDOWS,
                        [
                            (
                                TokenKind::Comment,
                                concat!(
                                    r"{ ",
                                    output_for_hello_world!(),
                                    r" in Borland Pascal 7 for MS-Windows}"
                                ),
                            ),
                            (
                                TokenKind::Plain,
                                r"

",
                            ),
                            (TokenKind::Keyword, r"PROGRAM"),
                            (
                                TokenKind::Plain,
                                r" HelloWorld;

",
                            ),
                            (TokenKind::Keyword, r"USES"),
                            (
                                TokenKind::Plain,
                                r"
  WinCRT;

",
                            ),
                            (TokenKind::Keyword, r"BEGIN"),
                            (
                                TokenKind::Plain,
                                r"
  InitWinCRT;
  WriteLn(",
                            ),
                            (
                                TokenKind::String,
                                concat!(r"'", output_for_hello_world!(), r"'"),
                            ),
                            (
                                TokenKind::Plain,
                                r");
  ReadLn;
  DoneWinCRT;
",
                            ),
                            (TokenKind::Keyword, r"END"),
                            (TokenKind::Plain, r"."),
                        ],
                    )
                },
            },
        ],
    };
