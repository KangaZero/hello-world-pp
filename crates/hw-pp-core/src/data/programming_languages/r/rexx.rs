//! Hello-world snippets for `ProgrammingLanguageName::Rexx`.
//!
//! Imported from leachim6/hello-world (`r/Rexx.rexx` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Imported from the Hello World Collection (`files/Rexx&nbsp;(simple)`, `files/Rexx&nbsp;(window)` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Rexx.rexx>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Rexx%26nbsp%3B%28simple%29>
//! - <http://helloworldcollection.de/#Rexx%C2%A0(simple)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Rexx%26nbsp%3B%28window%29>
//! - <http://helloworldcollection.de/#Rexx%C2%A0(window)>
//! - <https://www.wikidata.org/wiki/Q685820> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Procedural, Structured};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Rexx, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rexx => "Rexx",
    versions: [Rexx => "Rexx", RexxSimple => "Rexx (simple)", RexxWindow => "Rexx (window)"],
    extensions: [Rexx => "rexx"],
}

const REXX: &str = concat!(
    r#"<<hello.rexx>>=
Say ""#,
    output_for_hello_world!(),
    r#"""#
);
const REXX_SIMPLE: &str = concat!(
    r"/* ",
    output_for_hello_world!(),
    r" in Rexx, simple version (writes to standard output) */

say '",
    output_for_hello_world!(),
    r"'
exit"
);
const REXX_WINDOW: &str = concat!(
    r"/* Hallo World in Rexx, opens window */

call RxFuncAdd 'SysLoadFuncs', 'RexxUtil', 'SysLoadFuncs'
call SysLoadFuncs
call RxMessageBox '",
    output_for_hello_world!(),
    r"', '",
    output_for_hello_world!(),
    r" Window', 'OK', 'EXCLAMATION'
exit"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rexx,
        extensions: &[FileExtension::Rexx],
        categories: &[Scripting],
        families: &[Rexx],
        paradigms: &[Procedural, Structured],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Rexx,
                syntax: REXX,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        REXX,
                        [
                            (TokenKind::Plain, r"<<hello"),
                            (TokenKind::Punctuation, r"."),
                            (
                                TokenKind::Plain,
                                r"rexx>>=
",
                            ),
                            (TokenKind::Keyword, r"Say"),
                            (TokenKind::Plain, r" "),
                            (
                                TokenKind::String,
                                concat!(r#"""#, output_for_hello_world!(), r#"""#),
                            ),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::RexxSimple,
                syntax: REXX_SIMPLE,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        REXX_SIMPLE,
                        [
                            (
                                TokenKind::Comment,
                                concat!(
                                    r"/* ",
                                    output_for_hello_world!(),
                                    r" in Rexx, simple version (writes to standard output) */"
                                ),
                            ),
                            (
                                TokenKind::Plain,
                                r"

",
                            ),
                            (TokenKind::Keyword, r"say"),
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
                            (TokenKind::Keyword, r"exit"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::RexxWindow,
                syntax: REXX_WINDOW,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        REXX_WINDOW,
                        [
                            (
                                TokenKind::Comment,
                                r"/* Hallo World in Rexx, opens window */",
                            ),
                            (
                                TokenKind::Plain,
                                r"

call RxFuncAdd ",
                            ),
                            (TokenKind::String, r"'SysLoadFuncs'"),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (TokenKind::String, r"'RexxUtil'"),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (TokenKind::String, r"'SysLoadFuncs'"),
                            (
                                TokenKind::Plain,
                                r"
call SysLoadFuncs
call RxMessageBox ",
                            ),
                            (
                                TokenKind::String,
                                concat!(r"'", output_for_hello_world!(), r"'"),
                            ),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (
                                TokenKind::String,
                                concat!(r"'", output_for_hello_world!(), r" Window'"),
                            ),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (TokenKind::String, r"'OK'"),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (TokenKind::String, r"'EXCLAMATION'"),
                            (
                                TokenKind::Plain,
                                r"
",
                            ),
                            (TokenKind::Keyword, r"exit"),
                        ],
                    )
                },
            },
        ],
    };
