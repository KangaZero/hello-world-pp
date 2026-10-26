//! Hello-world snippets for `ProgrammingLanguageName::Asp`.
//!
//! Imported from leachim6/hello-world (`a/ASP.asp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Imported from the Hello World Collection (`files/ASP&nbsp;(C&#9839;).htm`, `files/ASP&nbsp;(JavaScript).txt`, `files/ASP&nbsp;(VBE).txt`, `files/ASP&nbsp;(VBS).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ASP.asp>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ASP%26nbsp%3B%28C%26%239839%3B%29.htm>
//! - <http://helloworldcollection.de/#ASP%C2%A0(C%E2%99%AF)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ASP%26nbsp%3B%28JavaScript%29.txt>
//! - <http://helloworldcollection.de/#ASP%C2%A0(JavaScript)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ASP%26nbsp%3B%28VBE%29.txt>
//! - <http://helloworldcollection.de/#ASP%C2%A0(VBE)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ASP%26nbsp%3B%28VBS%29.txt>
//! - <http://helloworldcollection.de/#ASP%C2%A0(VBS)>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Framework,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Asp => "ASP",
    versions: [Asp => "ASP", AspCU266f => "ASP (C♯)", AspJavascript => "ASP (JavaScript)", AspVbe => "ASP (VBE)", AspVbs => "ASP (VBS)"],
    extensions: [Asp => "asp"],
}

const ASP: &str = concat!(
    r#"<%@ Language= "VBScript" %>
<%
    Response.Write(""#,
    output_for_hello_world!(),
    r#"")
%>"#
);
const ASP_CU266F: &str = concat!(
    r"<!-- ",
    output_for_hello_world!(),
    r#" for ASP.NET using C# -->
<% @ Page Language="C#" %>
<% =""#,
    output_for_hello_world!(),
    r#"" %>"#
);
const ASP_JAVASCRIPT: &str = concat!(
    output_for_hello_world!(),
    r#" for Microsoft ASP (in JavaScript)

<%@ language="javascript" %>
<html><body>
<%
Response.Write('"#,
    output_for_hello_world!(),
    r"');
%>
</body></html>"
);
const ASP_VBE: &str = concat!(
    r"<!-- ",
    output_for_hello_world!(),
    r#" in ASP-VBE (Visual Basic Script Encided) -->
<html>
<script language="VBScript.Encode">#@~^HQAAAA==@#@&HdTAK6PrCsVKP    WMV[Zr@#@&HwcAAA==^#~@</script>
</html>"#
);
const ASP_VBS: &str = concat!(
    output_for_hello_world!(),
    r#" for Microsoft ASP (in VBScript)

<%@ language="vbscript" %>
<html><body>
<%
Response.write ""#,
    output_for_hello_world!(),
    r#""
%>
</body></html>"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Asp,
        extensions: &[FileExtension::Asp],
        categories: &[Framework],
        families: &[],
        paradigms: &[],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Asp,
                syntax: ASP,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        ASP,
                        [
                            (TokenKind::Keyword, r"<"),
                            (TokenKind::Plain, r"%@ Language"),
                            (TokenKind::Keyword, r"="),
                            (TokenKind::Plain, r" "),
                            (TokenKind::String, r#""VBScript""#),
                            (TokenKind::Plain, r" %"),
                            (TokenKind::Keyword, r">"),
                            (
                                TokenKind::Plain,
                                r"
",
                            ),
                            (TokenKind::Keyword, r"<"),
                            (
                                TokenKind::Plain,
                                r"%
    Response",
                            ),
                            (TokenKind::Punctuation, r"."),
                            (TokenKind::Function, r"Write"),
                            (TokenKind::Punctuation, r"("),
                            (
                                TokenKind::String,
                                concat!(r#"""#, output_for_hello_world!(), r#"""#),
                            ),
                            (TokenKind::Punctuation, r")"),
                            (
                                TokenKind::Plain,
                                r"
%",
                            ),
                            (TokenKind::Keyword, r">"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::AspCU266f,
                syntax: ASP_CU266F,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        ASP_CU266F,
                        [
                            (TokenKind::Keyword, r"<"),
                            (TokenKind::Plain, r"!"),
                            (TokenKind::Keyword, r"--"),
                            (
                                TokenKind::Plain,
                                concat!(r" ", output_for_hello_world!(), r" "),
                            ),
                            (TokenKind::Keyword, r"for"),
                            (TokenKind::Plain, r" ASP"),
                            (TokenKind::Punctuation, r"."),
                            (TokenKind::Plain, r"NET using C# "),
                            (TokenKind::Keyword, r"-->"),
                            (
                                TokenKind::Plain,
                                r"
",
                            ),
                            (TokenKind::Keyword, r"<"),
                            (TokenKind::Plain, r"% @ Page Language"),
                            (TokenKind::Keyword, r"="),
                            (TokenKind::String, r#""C#""#),
                            (TokenKind::Plain, r" %"),
                            (TokenKind::Keyword, r">"),
                            (
                                TokenKind::Plain,
                                r"
",
                            ),
                            (TokenKind::Keyword, r"<"),
                            (TokenKind::Plain, r"% "),
                            (TokenKind::Keyword, r"="),
                            (
                                TokenKind::String,
                                concat!(r#"""#, output_for_hello_world!(), r#"""#),
                            ),
                            (TokenKind::Plain, r" %"),
                            (TokenKind::Keyword, r">"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::AspJavascript,
                syntax: ASP_JAVASCRIPT,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        ASP_JAVASCRIPT,
                        [
                            (TokenKind::Plain, concat!(output_for_hello_world!(), r" ")),
                            (TokenKind::Keyword, r"for"),
                            (TokenKind::Plain, r" Microsoft ASP "),
                            (TokenKind::Punctuation, r"("),
                            (TokenKind::Plain, r"in JavaScript"),
                            (TokenKind::Punctuation, r")"),
                            (
                                TokenKind::Plain,
                                r"

",
                            ),
                            (TokenKind::Keyword, r"<"),
                            (TokenKind::Plain, r"%@ language"),
                            (TokenKind::Keyword, r"="),
                            (TokenKind::String, r#""javascript""#),
                            (TokenKind::Plain, r" %"),
                            (TokenKind::Keyword, r">"),
                            (
                                TokenKind::Plain,
                                r"
",
                            ),
                            (TokenKind::Keyword, r"<"),
                            (TokenKind::Plain, r"html><body"),
                            (TokenKind::Keyword, r">"),
                            (
                                TokenKind::Plain,
                                r"
",
                            ),
                            (TokenKind::Keyword, r"<"),
                            (
                                TokenKind::Plain,
                                r"%
Response",
                            ),
                            (TokenKind::Punctuation, r"."),
                            (TokenKind::Function, r"Write"),
                            (TokenKind::Punctuation, r"("),
                            (
                                TokenKind::Comment,
                                concat!(
                                    r"'",
                                    output_for_hello_world!(),
                                    r"');
"
                                ),
                            ),
                            (TokenKind::Plain, r"%"),
                            (TokenKind::Keyword, r">"),
                            (
                                TokenKind::Plain,
                                r"
",
                            ),
                            (TokenKind::Keyword, r"</"),
                            (TokenKind::Plain, r"body><"),
                            (TokenKind::Keyword, r"/"),
                            (TokenKind::Plain, r"html"),
                            (TokenKind::Keyword, r">"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::AspVbe,
                syntax: ASP_VBE,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        ASP_VBE,
                        [
                            (TokenKind::Keyword, r"<"),
                            (TokenKind::Plain, r"!"),
                            (TokenKind::Keyword, r"--"),
                            (
                                TokenKind::Plain,
                                concat!(r" ", output_for_hello_world!(), r" in ASP"),
                            ),
                            (TokenKind::Keyword, r"-"),
                            (TokenKind::Plain, r"VBE "),
                            (TokenKind::Punctuation, r"("),
                            (TokenKind::Plain, r"Visual Basic Script Encided"),
                            (TokenKind::Punctuation, r")"),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Keyword, r"-->"),
                            (
                                TokenKind::Plain,
                                r"
",
                            ),
                            (TokenKind::Keyword, r"<"),
                            (TokenKind::Plain, r"html"),
                            (TokenKind::Keyword, r">"),
                            (
                                TokenKind::Plain,
                                r"
",
                            ),
                            (TokenKind::Keyword, r"<"),
                            (TokenKind::Plain, r"script language"),
                            (TokenKind::Keyword, r"="),
                            (TokenKind::String, r#""VBScript.Encode""#),
                            (TokenKind::Keyword, r">"),
                            (TokenKind::Plain, r"#@~"),
                            (TokenKind::Keyword, r"^"),
                            (TokenKind::Plain, r"HQAAAA"),
                            (TokenKind::Keyword, r"=="),
                            (TokenKind::Plain, r"@#@"),
                            (TokenKind::Keyword, r"&"),
                            (
                                TokenKind::Plain,
                                r"HdTAK6PrCsVKP    WMV[Zr@#@&HwcAAA==^#~@</script>
",
                            ),
                            (TokenKind::Keyword, r"</"),
                            (TokenKind::Plain, r"html"),
                            (TokenKind::Keyword, r">"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::AspVbs,
                syntax: ASP_VBS,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        ASP_VBS,
                        [
                            (TokenKind::Plain, concat!(output_for_hello_world!(), r" ")),
                            (TokenKind::Keyword, r"for"),
                            (TokenKind::Plain, r" Microsoft ASP "),
                            (TokenKind::Punctuation, r"("),
                            (TokenKind::Plain, r"in VBScript"),
                            (TokenKind::Punctuation, r")"),
                            (
                                TokenKind::Plain,
                                r"

",
                            ),
                            (TokenKind::Keyword, r"<"),
                            (TokenKind::Plain, r"%@ language"),
                            (TokenKind::Keyword, r"="),
                            (TokenKind::String, r#""vbscript""#),
                            (TokenKind::Plain, r" %"),
                            (TokenKind::Keyword, r">"),
                            (
                                TokenKind::Plain,
                                r"
",
                            ),
                            (TokenKind::Keyword, r"<"),
                            (TokenKind::Plain, r"html><body"),
                            (TokenKind::Keyword, r">"),
                            (
                                TokenKind::Plain,
                                r"
",
                            ),
                            (TokenKind::Keyword, r"<"),
                            (
                                TokenKind::Plain,
                                r"%
Response",
                            ),
                            (TokenKind::Punctuation, r"."),
                            (TokenKind::Function, r"write"),
                            (TokenKind::Plain, r" "),
                            (
                                TokenKind::String,
                                concat!(r#"""#, output_for_hello_world!(), r#"""#),
                            ),
                            (
                                TokenKind::Plain,
                                r"
%",
                            ),
                            (TokenKind::Keyword, r">"),
                            (
                                TokenKind::Plain,
                                r"
",
                            ),
                            (TokenKind::Keyword, r"</"),
                            (TokenKind::Plain, r"body><"),
                            (TokenKind::Keyword, r"/"),
                            (TokenKind::Plain, r"html"),
                            (TokenKind::Keyword, r">"),
                        ],
                    )
                },
            },
        ],
    };
