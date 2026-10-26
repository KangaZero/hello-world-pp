//! Hello-world snippet for `ProgrammingLanguageName::Coral66`.
//!
//! Imported from leachim6/hello-world (`c/Coral 66.cor` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Coral%2066.cor>
//! - <https://www.wikidata.org/wiki/Q4117844> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Procedural, Structured};
use crate::data::programming_languages::{
    ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Coral66 => "Coral 66",
    versions: [Coral66 => "Coral 66"],
    extensions: [],
}

const CORAL66: &str = concat!(
    r#"'external' (
   'procedure' write (
      'value''integer', 'byte''array', 'value''integer');
)

'begin'
   'byte''array' Buf [1:12] := ""#,
    output_for_hello_world!(),
    r#"", 10;

   write (1, Buf, 12);
'end'"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Coral66,
        extensions: &[],
        categories: &[Compiled, Historical],
        families: &[],
        paradigms: &[Imperative, Procedural, Structured],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Coral66,
            syntax: CORAL66,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CORAL66,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"'external' (
   'procedure' write (
      'value''integer', 'byte''array', 'value''integer');
)

'begin'
   'byte''array' Buf [1:12] := ""#,
                            output_for_hello_world!(),
                            r#"", 10;

   write (1, Buf, 12);
'end'"#
                        ),
                    )],
                )
            },
        }],
    };
