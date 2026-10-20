//! Hello-world snippets for `ProgrammingLanguageName::Java`.
//!
//! Sources:
//! - <https://openjdk.org/jeps/512>
//! - <https://docs.oracle.com/javase/tutorial/getStarted/application/index.html>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Java%26nbsp%3B%28Mobile%29.txt>
//! - <http://helloworldcollection.de/#Java%C2%A0(Mobile)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Java%26nbsp%3B%28Servlet%29.txt>
//! - <http://helloworldcollection.de/#Java%C2%A0(Servlet)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Java%26nbsp%3B%28Swing%29.txt>
//! - <http://helloworldcollection.de/#Java%C2%A0(Swing)>
//!
//! The `Java (Mobile)`, `Java (Servlet)`, `Java (Swing)` versions are imported from the Hello World
//! Collection, MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{C, Java},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Java => "Java",
    versions: [Java17 => "Java17", Java25 => "Java25", JavaMobile => "Java (Mobile)", JavaServlet => "Java (Servlet)", JavaSwing => "Java (Swing)"],
    extensions: [Java => "java"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Java,
        extensions: &[FileExtension::Java],
        categories: &[Compiled],
        families: &[C, Java],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Java17,
                syntax: concat!(
                    r#"class Main {
    public static void main(String[] args) {
        System.out.println(""#,
                    output_for_hello_world!(),
                    r#"");
    }
}"#
                ),
                tokenized_syntax: &[
                    (TokenKind::Type, "class"),
                    (TokenKind::Plain, " Main "),
                    (TokenKind::Punctuation, "{"),
                    (TokenKind::Plain, "\n    "),
                    (TokenKind::Keyword, "public"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Keyword, "static"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Type, "void"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Function, "main"),
                    (TokenKind::Punctuation, "("),
                    (TokenKind::Plain, "String"),
                    (TokenKind::Keyword, "[]"),
                    (TokenKind::Plain, " args"),
                    (TokenKind::Punctuation, ")"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Punctuation, "{"),
                    (TokenKind::Plain, "\n        System"),
                    (TokenKind::Punctuation, "."),
                    (TokenKind::Plain, "out"),
                    (TokenKind::Punctuation, "."),
                    (TokenKind::Function, "println"),
                    (TokenKind::Punctuation, "("),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), "\""),
                    ),
                    (TokenKind::Punctuation, ");"),
                    (TokenKind::Plain, "\n    "),
                    (TokenKind::Punctuation, "}"),
                    (TokenKind::Plain, "\n"),
                    (TokenKind::Punctuation, "}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Java25,
                syntax: concat!(
                    r#"void main() {
    IO.println(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
                tokenized_syntax: &[
                    (TokenKind::Type, "void"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Function, "main"),
                    (TokenKind::Punctuation, "()"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Punctuation, "{"),
                    (TokenKind::Plain, "\n    IO"),
                    (TokenKind::Punctuation, "."),
                    (TokenKind::Function, "println"),
                    (TokenKind::Punctuation, "("),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), "\""),
                    ),
                    (TokenKind::Punctuation, ");"),
                    (TokenKind::Plain, "\n"),
                    (TokenKind::Punctuation, "}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::JavaMobile,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" on a mobile Java device

package helloworld;

import javax.microedition.midlet.*;
import javax.microedition.lcdui.*;

public class HelloWorld extends MIDlet {

  public HelloWorld()
  {
    Form form = new Form(""#,
                    output_for_hello_world!(),
                    r#"");
    form.append(""#,
                    output_for_hello_world!(),
                    r#"");
    Display.getDisplay(this).setCurrent(form);
  }

  protected void pauseApp() {  }
  protected void startApp() throws
    javax.microedition.midlet.MIDletStateChangeException {  }
  protected void destroyApp(boolean parm1) throws
    javax.microedition.midlet.MIDletStateChangeException {  }
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r" on a mobile Java device
"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"package"),
                    (TokenKind::Plain, r" helloworld"),
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
                    (TokenKind::Keyword, r"public"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Type, r"class"),
                    (TokenKind::Plain, r" HelloWorld "),
                    (TokenKind::Keyword, r"extends"),
                    (TokenKind::Plain, r" MIDlet "),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"

  ",
                    ),
                    (TokenKind::Keyword, r"public"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"HelloWorld"),
                    (TokenKind::Punctuation, r"()"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
    Form form ",
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
    form",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"append"),
                    (TokenKind::Punctuation, r"("),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
    Display",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"getDisplay"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"this"),
                    (TokenKind::Punctuation, r")."),
                    (TokenKind::Function, r"setCurrent"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"form"),
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
                    (TokenKind::Plain, r"  "),
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
                    (TokenKind::Keyword, r"throws"),
                    (
                        TokenKind::Plain,
                        r"
    javax.microedition.midlet",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Plain, r"MIDletStateChangeException "),
                    (TokenKind::Punctuation, r"{"),
                    (TokenKind::Plain, r"  "),
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
                    (TokenKind::Plain, r" parm1"),
                    (TokenKind::Punctuation, r")"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"throws"),
                    (
                        TokenKind::Plain,
                        r"
    javax.microedition.midlet",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Plain, r"MIDletStateChangeException "),
                    (TokenKind::Punctuation, r"{"),
                    (TokenKind::Plain, r"  "),
                    (TokenKind::Punctuation, r"}"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::JavaServlet,
                syntax: concat!(
                    r"import javax.servlet.*;
import javax.servlet.http.*;
import java.io.*;

//
// ",
                    output_for_hello_world!(),
                    r#" Java Servlet
//
public class HelloWorld extends HttpServlet {
public void service(HttpServletRequest request,
HttpServletResponse response)
throws IOException {

response.setContentType("text/html");
PrintWriter out = response.getWriter();

out.println("<html><body>");
out.println(""#,
                    output_for_hello_world!(),
                    r#"");
out.println("</body></html>");
}
}"#
                ),
                tokenized_syntax: &[
                    (TokenKind::Keyword, r"import"),
                    (TokenKind::Plain, r" javax"),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Plain, r"servlet.*"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"import"),
                    (TokenKind::Plain, r" javax.servlet"),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Plain, r"http.*"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"import"),
                    (TokenKind::Plain, r" java"),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Plain, r"io.*"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (
                        TokenKind::Comment,
                        concat!(
                            r"//
// ",
                            output_for_hello_world!(),
                            r" Java Servlet
//
"
                        ),
                    ),
                    (TokenKind::Keyword, r"public"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Type, r"class"),
                    (TokenKind::Plain, r" HelloWorld "),
                    (TokenKind::Keyword, r"extends"),
                    (TokenKind::Plain, r" HttpServlet "),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"public"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Type, r"void"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"service"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"HttpServletRequest request"),
                    (TokenKind::Punctuation, r","),
                    (
                        TokenKind::Plain,
                        r"
HttpServletResponse response",
                    ),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"throws"),
                    (TokenKind::Plain, r" IOException "),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"

response",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"setContentType"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::String, r#""text/html""#),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
PrintWriter out ",
                    ),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" response"),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"getWriter"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"

out",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"println"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::String, r#""<html><body>""#),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
out",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"println"),
                    (TokenKind::Punctuation, r"("),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
out",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"println"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::String, r#""</body></html>""#),
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
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::JavaSwing,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in Java using Swing GUI

class HelloWorldSwing {
  static public void main(String args[]) {
    javax.swing.JOptionPane.showMessageDialog(null,""#,
                    output_for_hello_world!(),
                    r#"");
  }
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r" in Java using Swing GUI
"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Type, r"class"),
                    (TokenKind::Plain, r" HelloWorldSwing "),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Keyword, r"static"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"public"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Type, r"void"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"main"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"String args"),
                    (TokenKind::Keyword, r"[]"),
                    (TokenKind::Punctuation, r")"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
    javax.swing",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Plain, r"JOptionPane"),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"showMessageDialog"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"null"),
                    (TokenKind::Punctuation, r","),
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
                    (TokenKind::Punctuation, r"}"),
                ],
            },
        ],
    };
