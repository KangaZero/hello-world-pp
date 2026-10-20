//! Hello-world snippet for `ProgrammingLanguageName::Alphabet`.
//!
//! Imported from leachim6/hello-world (`a/AlPhAbEt.abc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/AlPhAbEt.abc>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Alphabet => "AlPhAbEt",
    versions: [Alphabet => "AlPhAbEt"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Alphabet,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Alphabet,
            syntax: r"{

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
            tokenized_syntax: &[(
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
        }],
    };
