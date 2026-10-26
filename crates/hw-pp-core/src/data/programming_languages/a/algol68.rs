//! Hello-world snippet for `ProgrammingLanguageName::Algol68`.
//!
//! Imported from leachim6/hello-world (`a/ALGOL 68.algol68` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ALGOL%2068.algol68>
//! - <https://www.wikidata.org/wiki/Q2166735> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Concurrent, Imperative};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Algol,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Algol68 => "ALGOL 68",
    versions: [Algol68 => "ALGOL 68"],
    extensions: [Algol68 => "algol68"],
}

const ALGOL68: &str = concat!(
    r#"begin
  print((""#,
    output_for_hello_world!(),
    r#"",newline))
end"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Algol68,
        extensions: &[FileExtension::Algol68],
        categories: &[Compiled, Historical],
        families: &[Algol],
        paradigms: &[Imperative, Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Algol68,
            syntax: ALGOL68,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ALGOL68,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"begin
  print((""#,
                            output_for_hello_world!(),
                            r#"",newline))
end"#
                        ),
                    )],
                )
            },
        }],
    };
