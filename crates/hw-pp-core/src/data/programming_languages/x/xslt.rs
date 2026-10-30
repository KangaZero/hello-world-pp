//! Hello-world snippet for `ProgrammingLanguageName::Xslt`.
//!
//! Imported from leachim6/hello-world (`x/XSLT.xslt` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/XSLT.xslt>
//! - <https://www.wikidata.org/wiki/Q32110> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Declarative;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Xslt => "XSLT",
    versions: [Xslt => "XSLT"],
    extensions: [Xslt => "xslt"],
}

const XSLT: &str = concat!(
    r#"<?xml version="1.0" encoding="UTF-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
    <xsl:template match="/">
        <xsl:text>"#,
    output_for_hello_world!(),
    r"</xsl:text>
    </xsl:template>
</xsl:stylesheet>"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xslt,
        extensions: &[FileExtension::Xslt],
        categories: &[Markup],
        families: &[],
        paradigms: &[Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xslt,
            syntax: XSLT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    XSLT,
                    [
                        (TokenKind::Punctuation, r"<?"),
                        (TokenKind::Plain, r"xml version"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""1.0""#),
                        (TokenKind::Plain, r" encoding"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""UTF-8""#),
                        (TokenKind::Punctuation, r"?>"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"xsl"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"stylesheet version"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""1.0""#),
                        (TokenKind::Plain, r" xmlns"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"xsl"),
                        (TokenKind::Punctuation, r"="),
                        (
                            TokenKind::String,
                            r#""http://www.w3.org/1999/XSL/Transform""#,
                        ),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"xsl"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"template match"),
                        (TokenKind::Punctuation, r"="),
                        (TokenKind::String, r#""/""#),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Punctuation, r"<"),
                        (TokenKind::Plain, r"xsl"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"text"),
                        (TokenKind::Punctuation, r">"),
                        (TokenKind::Plain, output_for_hello_world!()),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"xsl"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"text"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"xsl"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"template"),
                        (TokenKind::Punctuation, r">"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"</"),
                        (TokenKind::Plain, r"xsl"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"stylesheet"),
                        (TokenKind::Punctuation, r">"),
                    ],
                )
            },
        }],
    };
