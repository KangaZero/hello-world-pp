//! Hello-world snippet for `ProgrammingLanguageName::Alphabet`.
//!
//! Imported from leachim6/hello-world (`a/AlPhAbEt.abc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/AlPhAbEt.abc>
//! - <https://esolangs.org/wiki/AlPhAbEt> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Alphabet => "AlPhAbEt",
    versions: [Alphabet => "AlPhAbEt"],
    extensions: [],
}

const ALPHABET: &str = r"{

  0>,

  1>. 2>, 3>. 4>. 5>, 6>. 7>. 8>. 9>,
  1>. 2>, 3>, 4>. 5>. 6>, 7>. 8>, 9>,
  1>. 2>, 3>, 4>. 5>, 6>, 7>. 8>. 9>,
  1>. 2>, 3>, 4>. 5>, 6>, 7>. 8>. 9>,
  1>. 2>, 3>, 4>. 5>, 6>, 7>, 8>, 9>,
  1>. 2>. 3>, 4>. 5>. 6>. 7>. 8>. 9>,
  1>. 2>, 3>. 4>, 5>. 6>, 7>, 8>, 9>,
  1>. 2>, 3>, 4>. 5>, 6>, 7>, 8>, 9>,
  1>. 2>, 3>, 4>, 5>. 6>. 7>, 8>. 9>,
  1>. 2>, 3>, 4>. 5>, 6>, 7>. 8>. 9>,
  1>. 2>, 3>, 4>. 5>. 6>, 7>. 8>. 9>,

}";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Alphabet,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Alphabet,
            syntax: ALPHABET,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ALPHABET,
                    [(
                        TokenKind::Plain,
                        r"{

  0>,

  1>. 2>, 3>. 4>. 5>, 6>. 7>. 8>. 9>,
  1>. 2>, 3>, 4>. 5>. 6>, 7>. 8>, 9>,
  1>. 2>, 3>, 4>. 5>, 6>, 7>. 8>. 9>,
  1>. 2>, 3>, 4>. 5>, 6>, 7>. 8>. 9>,
  1>. 2>, 3>, 4>. 5>, 6>, 7>, 8>, 9>,
  1>. 2>. 3>, 4>. 5>. 6>. 7>. 8>. 9>,
  1>. 2>, 3>. 4>, 5>. 6>, 7>, 8>, 9>,
  1>. 2>, 3>, 4>. 5>, 6>, 7>, 8>, 9>,
  1>. 2>, 3>, 4>, 5>. 6>. 7>, 8>. 9>,
  1>. 2>, 3>, 4>. 5>, 6>, 7>. 8>. 9>,
  1>. 2>, 3>, 4>. 5>. 6>, 7>. 8>. 9>,

}",
                    )],
                )
            },
        }],
    };
