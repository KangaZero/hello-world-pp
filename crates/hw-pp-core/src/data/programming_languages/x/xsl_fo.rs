//! Hello-world snippet for `ProgrammingLanguageName::XslFo`.
//!
//! Imported from the Hello World Collection (`files/XSL-FO.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/XSL-FO.txt>
//! - <http://helloworldcollection.de/#XSL-FO>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: XslFo => "XSL-FO",
    versions: [XslFo => "XSL-FO"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::XslFo,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::XslFo,
            syntax: concat!(
                r#"<?xml version="1.0" encoding="utf-8"?>
<!-- "#,
                output_for_hello_world!(),
                r#" in XSL-FO -->
<fo:root xmlns:fo="http://www.w3.org/1999/XSL/Format">
    <fo:layout-master-set>
        <fo:simple-page-master master-name="LetterPage" page-width="8.5in" page-height="11in">?
            <fo:region-body region-name="PageBody" margin="0.7in"/>
        </fo:simple-page-master>
    </fo:layout-master-set>
    <fo:page-sequence master-reference="LetterPage">
        <fo:flow flow-name="PageBody">
            <fo:block font-size="12pt" font-family="courier">"#,
                output_for_hello_world!(),
                r"</fo:block>
        </fo:flow>
    </fo:page-sequence>
</fo:root>"
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"<"),
                (TokenKind::Plain, r"?xml version="),
                (TokenKind::String, r#""1.0""#),
                (TokenKind::Plain, r" encoding="),
                (TokenKind::String, r#""utf-8""#),
                (TokenKind::Plain, r"?"),
                (TokenKind::Keyword, r">"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (
                    TokenKind::Comment,
                    concat!(r"<!-- ", output_for_hello_world!(), r" in XSL-FO -->"),
                ),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"<fo:root"),
                (TokenKind::Plain, r" xmlns:fo="),
                (TokenKind::String, r#""http://www.w3.org/1999/XSL/Format""#),
                (TokenKind::Keyword, r">"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"<fo:layout-master-set>"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"<fo:simple-page-master"),
                (TokenKind::Plain, r" master-name="),
                (TokenKind::String, r#""LetterPage""#),
                (TokenKind::Plain, r" page-width="),
                (TokenKind::String, r#""8.5in""#),
                (TokenKind::Plain, r" page-height="),
                (TokenKind::String, r#""11in""#),
                (TokenKind::Keyword, r">"),
                (
                    TokenKind::Plain,
                    r"?
            ",
                ),
                (TokenKind::Keyword, r"<fo:region-body"),
                (TokenKind::Plain, r" region-name="),
                (TokenKind::String, r#""PageBody""#),
                (TokenKind::Plain, r" margin="),
                (TokenKind::String, r#""0.7in""#),
                (TokenKind::Keyword, r"/>"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"</fo:simple-page-master>"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"</fo:layout-master-set>"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"<fo:page-sequence"),
                (TokenKind::Plain, r" master-reference="),
                (TokenKind::String, r#""LetterPage""#),
                (TokenKind::Keyword, r">"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"<fo:flow"),
                (TokenKind::Plain, r" flow-name="),
                (TokenKind::String, r#""PageBody""#),
                (TokenKind::Keyword, r">"),
                (
                    TokenKind::Plain,
                    r"
            ",
                ),
                (TokenKind::Keyword, r"<fo:block"),
                (TokenKind::Plain, r" font-size="),
                (TokenKind::String, r#""12pt""#),
                (TokenKind::Plain, r" font-family="),
                (TokenKind::String, r#""courier""#),
                (TokenKind::Keyword, r">"),
                (TokenKind::Plain, output_for_hello_world!()),
                (TokenKind::Keyword, r"</fo:block>"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"</fo:flow>"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"</fo:page-sequence>"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"</fo:root>"),
            ],
        }],
    };
