//! Hello-world snippet for `ProgrammingLanguageName::B`.
//!
//! Imported from leachim6/hello-world (`b/B.b` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/B.b>
//! - <https://www.wikidata.org/wiki/Q797302> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Procedural;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: B => "B",
    versions: [B => "B"],
    extensions: [B => "b"],
}

const B: &str = r"main() {
    putchar ('Hell'); putchar ('o Wo'); putchar ('rld'); putchar ('*n');
}";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::B,
        extensions: &[FileExtension::B],
        categories: &[Compiled, Historical],
        families: &[],
        paradigms: &[Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::B,
            syntax: B,
            tokenized_syntax: &const {
                checked_token_lengths(
                    B,
                    [(
                        TokenKind::Plain,
                        r"main() {
    putchar ('Hell'); putchar ('o Wo'); putchar ('rld'); putchar ('*n');
}",
                    )],
                )
            },
        }],
    };
