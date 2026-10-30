//! Hello-world snippet for `ProgrammingLanguageName::Svg`.
//!
//! Imported from leachim6/hello-world (`s/SVG.svg` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SVG.svg>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Svg => "SVG",
    versions: [Svg => "SVG"],
    extensions: [Svg => "svg"],
}

const SVG: &str = concat!(
    r#"<?xml version="1.0" encoding="utf-8" standalone="no"?>
<svg width="240" height="100" viewBox="0 0 240 100" zoomAndPan="disable"
     xmlns="http://www.w3.org/2000/svg"  xmlns:xlink="http://www.w3.org/1999/xlink">
  <title>"#,
    output_for_hello_world!(),
    r#"</title>
    <g>
      <text x="10" y="50">"#,
    output_for_hello_world!(),
    r#"</text>
      <animate attributeName='opacity' values='0;1' dur='4s' fill='freeze' begin="0s"/>
    </g>
</svg>"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Svg,
        extensions: &[FileExtension::Svg],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Svg,
            syntax: SVG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SVG,
                    [
                        (TokenKind::Punctuation, r"<?"),
                        (TokenKind::Plain, r"xml version"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""1.0""#),
                        (TokenKind::Plain, r" encoding"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""utf-8""#),
                        (TokenKind::Plain, r" standalone"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""no""#),
                        (TokenKind::Punctuation, r"?>"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"svg width"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""240""#),
                        (TokenKind::Plain, r" height"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""100""#),
                        (TokenKind::Plain, r" viewBox"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""0 0 240 100""#),
                        (TokenKind::Plain, r" zoomAndPan"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""disable""#),
                        (
                            TokenKind::Plain,
                            r"
     xmlns",
                        ),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""http://www.w3.org/2000/svg""#),
                        (TokenKind::Plain, r"  xmlns"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"xlink"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""http://www.w3.org/1999/xlink""#),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"title"),
                        (TokenKind::Punctuation, r">"),
                        (TokenKind::Plain, output_for_hello_world!()),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"title"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"g"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
      ",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"text x"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""10""#),
                        (TokenKind::Plain, r" y"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""50""#),
                        (TokenKind::Punctuation, r">"),
                        (TokenKind::Plain, output_for_hello_world!()),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"text"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
      ",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"animate attributeName"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r"'opacity'"),
                        (TokenKind::Plain, r" values"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r"'0;1'"),
                        (TokenKind::Plain, r" dur"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r"'4s'"),
                        (TokenKind::Plain, r" fill"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r"'freeze'"),
                        (TokenKind::Plain, r" begin"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""0s""#),
                        (TokenKind::Punctuation, r"/>"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"g"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"svg"),
                        (TokenKind::Punctuation, r">"),
                    ],
                )
            },
        }],
    };
