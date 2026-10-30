//! Hello-world snippet for `ProgrammingLanguageName::Dotlang`.
//!
//! Imported from leachim6/hello-world (`d/Dotlang` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Dotlang>
//! - <https://esolangs.org/wiki/Dotlang> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::TwoDimensional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Dotlang => "Dotlang",
    versions: [Dotlang => "Dotlang"],
    extensions: [],
}

const DOTLANG: &str = concat!(
    r"•v
 #`",
    output_for_hello_world!(),
    r"`
 #"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dotlang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dotlang,
            syntax: DOTLANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DOTLANG,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"•v
 #`",
                            output_for_hello_world!(),
                            r"`
 #"
                        ),
                    )],
                )
            },
        }],
    };
