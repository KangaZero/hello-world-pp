//! Hello-world snippet for `ProgrammingLanguageName::NextJs`.
//!
//! Imported from leachim6/hello-world (PR #1805 `n/Next.js`, PR #1805 `n/Next.tsx`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: NextJs => "Next.js",
    versions: [NextJsJs => "Next.js (.js)", NextJsTsx => "Next.js (.tsx)"],
    extensions: [Tsx => "tsx"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::NextJs,
        extensions: &[FileExtension::Tsx],
        categories: &[],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::NextJsJs,
                syntax: concat!(
                    r"export default function Home() {
  return <h1>",
                    output_for_hello_world!(),
                    r"</h1>;
}"
                ),
                tokenized_syntax: &[
                    (TokenKind::Keyword, r"export"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"default"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"function"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"Home"),
                    (TokenKind::Punctuation, r"()"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"<"),
                    (TokenKind::Plain, r"h1"),
                    (TokenKind::Keyword, r">"),
                    (TokenKind::Plain, output_for_hello_world!()),
                    (TokenKind::Keyword, r"<"),
                    (
                        TokenKind::String,
                        r"/h1>;
}",
                    ),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::NextJsTsx,
                syntax: concat!(
                    r"export default function Home(): JSX.Element {
  return <h1>",
                    output_for_hello_world!(),
                    r"</h1>;
}"
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
                        r"export default function Home(): JSX.Element {
  return <h1>",
                        output_for_hello_world!(),
                        r"</h1>;
}"
                    ),
                )],
            },
        ],
    };
