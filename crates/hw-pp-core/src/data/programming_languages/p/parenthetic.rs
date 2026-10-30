//! Hello-world snippet for `ProgrammingLanguageName::Parenthetic`.
//!
//! Imported from leachim6/hello-world (`p/Parenthetic.p` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Parenthetic.p>
//! - <https://esolangs.org/wiki/Parenthetic> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Parenthetic => "Parenthetic",
    versions: [Parenthetic => "Parenthetic"],
    extensions: [P => "p"],
}

const PARENTHETIC: &str = r"Parenthetic - https://github.com/cammckinnon/Parenthetic

((()()())(()(()()))((()(()))((())()()()()()()())((()()(()))((())()()()()()()()()())
((())()()()()()()()()()()))))((()()())(()(()()()))((()(())(())())((())()()()()()()(
)()()()()()()()()()()()()()()()()()()()()()()()()())))((()(()))((()(())(())())((()(
()))(()(()()))((())()()()()()()())))((()(())(())())((()(()))(()(()()))((())()()()()
)))((()(())(())())((()(()))(()(()()))((())()()()()()()()()()()())))((()(())(())())(
(()(()))(()(()()))((())()()()()()()()()()()())))((()(())(())())((()(()))(()(()()))(
(())()()()()()()()()()()()()()())))(()(()()()))((()(())(())())((()(()))(()(()()))((
())()()()()()()()()()()()()()()()()()()()()()())))((()(())(())())((()(()))(()(()())
)((())()()()()()()()()()()()()()())))((()(())(())())((()(()))(()(()()))((())()()()(
)()()()()()()()()()()()()())))((()(())(())())((()(()))(()(()()))((())()()()()()()()
()()()())))((()(())(())())((()(()))(()(()()))((())()()()))))";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Parenthetic,
        extensions: &[FileExtension::P],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Parenthetic,
            syntax: PARENTHETIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PARENTHETIC,
                    [(
                        TokenKind::Plain,
                        r"Parenthetic - https://github.com/cammckinnon/Parenthetic

((()()())(()(()()))((()(()))((())()()()()()()())((()()(()))((())()()()()()()()()())
((())()()()()()()()()()()))))((()()())(()(()()()))((()(())(())())((())()()()()()()(
)()()()()()()()()()()()()()()()()()()()()()()()()())))((()(()))((()(())(())())((()(
()))(()(()()))((())()()()()()()())))((()(())(())())((()(()))(()(()()))((())()()()()
)))((()(())(())())((()(()))(()(()()))((())()()()()()()()()()()())))((()(())(())())(
(()(()))(()(()()))((())()()()()()()()()()()())))((()(())(())())((()(()))(()(()()))(
(())()()()()()()()()()()()()()())))(()(()()()))((()(())(())())((()(()))(()(()()))((
())()()()()()()()()()()()()()()()()()()()()()())))((()(())(())())((()(()))(()(()())
)((())()()()()()()()()()()()()()())))((()(())(())())((()(()))(()(()()))((())()()()(
)()()()()()()()()()()()()())))((()(())(())())((()(()))(()(()()))((())()()()()()()()
()()()())))((()(())(())())((()(()))(()(()()))((())()()()))))",
                    )],
                )
            },
        }],
    };
