//! Hello-world snippet for `ProgrammingLanguageName::J2me`.
//!
//! Imported from leachim6/hello-world (PR #1679 `j/j2me.java`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1679>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: J2me => "J2ME",
    versions: [J2me => "J2ME"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::J2me,
        extensions: &[],
        categories: &[Framework],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::J2me,
            syntax: concat!(
                r#"import javax.microedition.lcdui.*;
import javax.microedition.midlet.*;

public class HelloMIDlet extends MIDlet {
	private Display display;
	private Form helloFrm;

	public HelloMIDlet() {
		helloFrm = new Form(""#,
                output_for_hello_world!(),
                r#"");
	}

	protected void startApp() {
		display = Display.getDisplay(this);
		display.setCurrent(helloFrm);
	}

	protected void pauseApp() {
	}

	protected void destroyApp(boolean unconditional) {
	}
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"import"),
                (TokenKind::Plain, r" javax.microedition"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"lcdui.*"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"import"),
                (TokenKind::Plain, r" javax.microedition"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"midlet.*"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"public"),
                (TokenKind::Plain, r" "),
                (TokenKind::Type, r"class"),
                (TokenKind::Plain, r" HelloMIDlet "),
                (TokenKind::Keyword, r"extends"),
                (TokenKind::Plain, r" MIDlet "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Keyword, r"private"),
                (TokenKind::Plain, r" Display display"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Keyword, r"private"),
                (TokenKind::Plain, r" Form helloFrm"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"

	",
                ),
                (TokenKind::Keyword, r"public"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"HelloMIDlet"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
		helloFrm ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"new"),
                (TokenKind::Plain, r" Form"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Punctuation, r"}"),
                (
                    TokenKind::Plain,
                    r"

	",
                ),
                (TokenKind::Keyword, r"protected"),
                (TokenKind::Plain, r" "),
                (TokenKind::Type, r"void"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"startApp"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
		display ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" Display"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"getDisplay"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"this"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
		display",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"setCurrent"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"helloFrm"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Punctuation, r"}"),
                (
                    TokenKind::Plain,
                    r"

	",
                ),
                (TokenKind::Keyword, r"protected"),
                (TokenKind::Plain, r" "),
                (TokenKind::Type, r"void"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"pauseApp"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Punctuation, r"}"),
                (
                    TokenKind::Plain,
                    r"

	",
                ),
                (TokenKind::Keyword, r"protected"),
                (TokenKind::Plain, r" "),
                (TokenKind::Type, r"void"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"destroyApp"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Type, r"boolean"),
                (TokenKind::Plain, r" unconditional"),
                (TokenKind::Punctuation, r")"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Punctuation, r"}"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };
