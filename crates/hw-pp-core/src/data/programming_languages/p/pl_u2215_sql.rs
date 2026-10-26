//! Hello-world snippet for `ProgrammingLanguageName::PlU2215Sql`.
//!
//! Imported from leachim6/hello-world (`p/PL∕SQL.pls` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PL%E2%88%95SQL.pls>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Query,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Ada, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: PlU2215Sql => "PL∕SQL",
    versions: [PlU2215Sql => "PL∕SQL"],
    extensions: [Pls => "pls"],
}

const PL_U2215_SQL: &str = concat!(
    r"BEGIN
  dbms_output.put_line('",
    output_for_hello_world!(),
    r"');
END;
/"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PlU2215Sql,
        extensions: &[FileExtension::Pls],
        categories: &[Query],
        families: &[Ada],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PlU2215Sql,
            syntax: PL_U2215_SQL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PL_U2215_SQL,
                    [
                        (TokenKind::Keyword, r"BEGIN"),
                        (
                            TokenKind::Plain,
                            r"
  dbms_output",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Keyword, r"put_line"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"END"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
/",
                        ),
                    ],
                )
            },
        }],
    };
