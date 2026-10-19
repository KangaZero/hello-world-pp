//! Hello-world snippets for `ProgrammingLanguageName::C`.
//!
//! Sources:
//! - <https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1256.pdf>
//! - <https://en.wikipedia.org/wiki/C99>
//! - <https://en.cppreference.com/c/io/printf>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%26nbsp%3B%28ANSI%29>
//! - <http://helloworldcollection.de/#C%C2%A0(ANSI)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%26nbsp%3B%28Amiga%26nbsp%3BAnywhere%29.txt>
//! - <http://helloworldcollection.de/#C%C2%A0(Amiga%C2%A0Anywhere)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%26nbsp%3B%28Curses%29>
//! - <http://helloworldcollection.de/#C%C2%A0(Curses)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%26nbsp%3B%28GEM%29>
//! - <http://helloworldcollection.de/#C%C2%A0(GEM)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%26nbsp%3B%28Intuition%29>
//! - <http://helloworldcollection.de/#C%C2%A0(Intuition)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%26nbsp%3B%28K%26amp%3BR%29>
//! - <http://helloworldcollection.de/#C%C2%A0(K%26R)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%26nbsp%3B%28OpenGL%29.txt>
//! - <http://helloworldcollection.de/#C%C2%A0(OpenGL)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%26nbsp%3B%28PresentationManager%29>
//! - <http://helloworldcollection.de/#C%C2%A0(PresentationManager)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%26nbsp%3B%28Windows%29>
//! - <http://helloworldcollection.de/#C%C2%A0(Windows)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%26nbsp%3B%28X11%26nbsp%3BAthena%29>
//! - <http://helloworldcollection.de/#C%C2%A0(X11%C2%A0Athena)>
//!
//! The `C (ANSI)`, `C (Amiga Anywhere)`, `C (Curses)`, `C (GEM)`, `C (Intuition)`, `C (K&R)`,
//! `C (OpenGL)`, `C (PresentationManager)`, `C (Windows)`, `C (X11 Athena)` versions are imported
//! from the Hello World Collection, MIT licensed; see
//! `programming_languages/LICENSE-helloworldcollection`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: C => "C",
    versions: [C99 => "C99", CAnsi => "C (ANSI)", CAmigaAnywhere => "C (Amiga Anywhere)", CCurses => "C (Curses)", CGem => "C (GEM)", CIntuition => "C (Intuition)", CKAndR => "C (K&R)", COpengl => "C (OpenGL)", CPresentationmanager => "C (PresentationManager)", CWindows => "C (Windows)", CX11Athena => "C (X11 Athena)"],
    extensions: [C => "c", H => "h"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::C,
        extensions: &[FileExtension::C, FileExtension::H],
        categories: &[Compiled],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::C99,
                syntax: concat!(
                    r#"#include <stdio.h>

int main(void) {
    printf(""#,
                    output_for_hello_world!(),
                    r#"\n");
    return 0;
}"#
                ),
                tokenized_syntax: &[
                    (TokenKind::Keyword, "#include"),
                    (TokenKind::Plain, " "),
                    (TokenKind::String, "<stdio.h>"),
                    (TokenKind::Plain, "\n\n"),
                    (TokenKind::Type, "int"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Function, "main"),
                    (TokenKind::Punctuation, "("),
                    (TokenKind::Type, "void"),
                    (TokenKind::Punctuation, ")"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Punctuation, "{"),
                    (TokenKind::Plain, "\n    "),
                    (TokenKind::Function, "printf"),
                    (TokenKind::Punctuation, "("),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), r#"\n""#),
                    ),
                    (TokenKind::Punctuation, ");"),
                    (TokenKind::Plain, "\n    "),
                    (TokenKind::Keyword, "return"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Number, "0"),
                    (TokenKind::Punctuation, ";"),
                    (TokenKind::Plain, "\n"),
                    (TokenKind::Punctuation, "}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CAnsi,
                syntax: concat!(
                    r"/* ",
                    output_for_hello_world!(),
                    r#" in C, Ansi-style */

#include <stdio.h>
#include <stdlib.h>

int main(void)
{
  puts(""#,
                    output_for_hello_world!(),
                    r#"");
  return EXIT_SUCCESS;
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(r"/* ", output_for_hello_world!(), r" in C, Ansi-style */"),
                    ),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<stdio.h>"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<stdlib.h>"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"main"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Type, r"void"),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Function, r"puts"),
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
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" EXIT_SUCCESS"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CAmigaAnywhere,
                syntax: concat!(
                    r"/* ",
                    output_for_hello_world!(),
                    r#" in C for Amiga Anywhere 2 (AA2) */

#include <aa.h>

int aaMain(int argc, char **argv)
{
   aaOpenDisplay(200, 200, 16, ""#,
                    output_for_hello_world!(),
                    r#"", FAA_DISPLAY_WINDOW);
   aaDrawString(AA_DISPLAY_PIXMAP, ""#,
                    output_for_hello_world!(),
                    r#"", 20, 20, AA_DEFAULT_FONT, 0xffff00, 0, FAA_FONT_INK, -1);
   aaUpdate();
   aaWaitInput();
   return 0;
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"/* ",
                            output_for_hello_world!(),
                            r" in C for Amiga Anywhere 2 (AA2) */"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<aa.h>"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"aaMain"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" argc"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Type, r"char"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"**"),
                    (TokenKind::Plain, r"argv"),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
   ",
                    ),
                    (TokenKind::Function, r"aaOpenDisplay"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Number, r"200"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"200"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"16"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" FAA_DISPLAY_WINDOW"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
   ",
                    ),
                    (TokenKind::Function, r"aaDrawString"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"AA_DISPLAY_PIXMAP"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"20"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"20"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" AA_DEFAULT_FONT"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0xffff00"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" FAA_FONT_INK"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"-"),
                    (TokenKind::Number, r"1"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
   ",
                    ),
                    (TokenKind::Function, r"aaUpdate"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
   ",
                    ),
                    (TokenKind::Function, r"aaWaitInput"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
   ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CCurses,
                syntax: concat!(
                    r"/* ",
                    output_for_hello_world!(),
                    r#" in C for Curses */

#include <curses.h>
main()
{
  initscr();
  addstr(""#,
                    output_for_hello_world!(),
                    r#"\n");
  refresh();
  endwin();
  return 0;
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(r"/* ", output_for_hello_world!(), r" in C for Curses */"),
                    ),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<curses.h>"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Function, r"main"),
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
  ",
                    ),
                    (TokenKind::Function, r"initscr"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Function, r"addstr"),
                    (TokenKind::Punctuation, r"("),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                    ),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Function, r"refresh"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Function, r"endwin"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CGem,
                syntax: concat!(
                    r"/* ",
                    output_for_hello_world!(),
                    r#" for C with GEM */

#include <aes.h>
main()
{
  appl_init();
  form_alert(1,"[0]["#,
                    output_for_hello_world!(),
                    r#"][Ok]");
  appl_exit();
  return 0;
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(r"/* ", output_for_hello_world!(), r" for C with GEM */"),
                    ),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<aes.h>"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Function, r"main"),
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
  ",
                    ),
                    (TokenKind::Function, r"appl_init"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Function, r"form_alert"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Number, r"1"),
                    (TokenKind::Punctuation, r","),
                    (
                        TokenKind::String,
                        concat!(r#""[0]["#, output_for_hello_world!(), r#"][Ok]""#),
                    ),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Function, r"appl_exit"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CIntuition,
                syntax: concat!(
                    r"/* ",
                    output_for_hello_world!(),
                    r#" in C for Intution (Amiga GUI) */

#include <intuition/intuition.h>

struct IntuitionBase *IntuitionBase = NULL;

struct IntuiText hello_text = {-1,-1,JAM1,0,0,NULL,""#,
                    output_for_hello_world!(),
                    r#"",NULL };
struct IntuiText ok_text    = {-1,-1,JAM1,0,0,NULL,"Ok",NULL };

void main(void)
{
   IntuitionBase = (struct IntuitionBase *)
                   OpenLibrary("intuition.library", 0);
   AutoRequest(NULL, &hello_text, NULL, &ok_text, NULL, NULL, 100, 50);
   CloseLibrary(IntuitionBase);
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"/* ",
                            output_for_hello_world!(),
                            r" in C for Intution (Amiga GUI) */"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<intuition/intuition.h>"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"struct"),
                    (TokenKind::Plain, r" IntuitionBase "),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r"IntuitionBase "),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" NULL"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"struct"),
                    (TokenKind::Plain, r" IntuiText hello_text "),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"{"),
                    (TokenKind::Keyword, r"-"),
                    (TokenKind::Number, r"1"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Keyword, r"-"),
                    (TokenKind::Number, r"1"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r"JAM1"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r"NULL"),
                    (TokenKind::Punctuation, r","),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r"NULL "),
                    (TokenKind::Punctuation, r"};"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Type, r"struct"),
                    (TokenKind::Plain, r" IntuiText ok_text    "),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"{"),
                    (TokenKind::Keyword, r"-"),
                    (TokenKind::Number, r"1"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Keyword, r"-"),
                    (TokenKind::Number, r"1"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r"JAM1"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r"NULL"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::String, r#""Ok""#),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r"NULL "),
                    (TokenKind::Punctuation, r"};"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"void"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"main"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Type, r"void"),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
   IntuitionBase ",
                    ),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Type, r"struct"),
                    (TokenKind::Plain, r" IntuitionBase "),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"
                   ",
                    ),
                    (TokenKind::Function, r"OpenLibrary"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::String, r#""intuition.library""#),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
   ",
                    ),
                    (TokenKind::Function, r"AutoRequest"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"NULL"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"&"),
                    (TokenKind::Plain, r"hello_text"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" NULL"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"&"),
                    (TokenKind::Plain, r"ok_text"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" NULL"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" NULL"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"100"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"50"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
   ",
                    ),
                    (TokenKind::Function, r"CloseLibrary"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"IntuitionBase"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CKAndR,
                syntax: concat!(
                    r"/* ",
                    output_for_hello_world!(),
                    r#" in C, K&R-style */

main()
{
  puts(""#,
                    output_for_hello_world!(),
                    r#"");
  return 0;
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(r"/* ", output_for_hello_world!(), r" in C, K&R-style */"),
                    ),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Function, r"main"),
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
  ",
                    ),
                    (TokenKind::Function, r"puts"),
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
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::COpengl,
                syntax: concat!(
                    r#"/* ""#,
                    output_for_hello_world!(),
                    r#"" in C using OGL - Open Graphics Library */

#include <GL/glut.h>
#define font GLUT_BITMAP_HELVETICA_18
#define tx ""#,
                    output_for_hello_world!(),
                    r#""

void text(void)
{
 char *p, tex[] = tx;
 p = tex;
 glColor3d(1.0, 1.0, 0.0);
 glRasterPos2d(-.5, 0.);
 while(*p) glutBitmapCharacter(font, *p++);
}

void display()
{
 glClear(GL_COLOR_BUFFER_BIT);
 text();
 glFlush();
}

void reshape(int width, int height)
{
 glViewport(0, 0, width, height);
 glMatrixMode(GL_PROJECTION);
 glLoadIdentity();
 glOrtho(-1, 1, -1, 1, -1, 1);
 glMatrixMode(GL_MODELVIEW);
 display();
}

int main(int argc, char **argv)
{
 glutInit(&argc, argv);
 glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
 glutInitWindowPosition(50, 50);
 glutInitWindowSize(500, 500);
 glutCreateWindow(""#,
                    output_for_hello_world!(),
                    r#" OpenGL");
 glClearColor(0,0,0,0);
 glutDisplayFunc(display);
 glutReshapeFunc(reshape);
 glutMainLoop();
 return 0;
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r#"/* ""#,
                            output_for_hello_world!(),
                            r#"" in C using OGL - Open Graphics Library */"#
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<GL/glut.h>"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#define"),
                    (
                        TokenKind::Plain,
                        r" font GLUT_BITMAP_HELVETICA_18
",
                    ),
                    (TokenKind::Keyword, r"#define"),
                    (TokenKind::Plain, r" tx "),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"void"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"text"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Type, r"void"),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Type, r"char"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r"p"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" tex"),
                    (TokenKind::Punctuation, r"[]"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" tx"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
 p ",
                    ),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" tex"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glColor3d"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Number, r"1.0"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"1.0"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0.0"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glRasterPos2d"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Keyword, r"-"),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Number, r"5"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r".);"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Keyword, r"while"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r"p"),
                    (TokenKind::Punctuation, r")"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"glutBitmapCharacter"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"font"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r"p"),
                    (TokenKind::Keyword, r"++"),
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
                    (TokenKind::Type, r"void"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"display"),
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
 ",
                    ),
                    (TokenKind::Function, r"glClear"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"GL_COLOR_BUFFER_BIT"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"text"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glFlush"),
                    (TokenKind::Punctuation, r"();"),
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
                    (TokenKind::Type, r"void"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"reshape"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" width"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" height"),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glViewport"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" width"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" height"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glMatrixMode"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"GL_PROJECTION"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glLoadIdentity"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glOrtho"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Keyword, r"-"),
                    (TokenKind::Number, r"1"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"1"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"-"),
                    (TokenKind::Number, r"1"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"1"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"-"),
                    (TokenKind::Number, r"1"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"1"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glMatrixMode"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"GL_MODELVIEW"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"display"),
                    (TokenKind::Punctuation, r"();"),
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
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"main"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" argc"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Type, r"char"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"**"),
                    (TokenKind::Plain, r"argv"),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glutInit"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Keyword, r"&"),
                    (TokenKind::Plain, r"argc"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" argv"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glutInitDisplayMode"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"GLUT_SINGLE "),
                    (TokenKind::Keyword, r"|"),
                    (TokenKind::Plain, r" GLUT_RGB"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glutInitWindowPosition"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Number, r"50"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"50"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glutInitWindowSize"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Number, r"500"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"500"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glutCreateWindow"),
                    (TokenKind::Punctuation, r"("),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#" OpenGL""#),
                    ),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glClearColor"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glutDisplayFunc"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"display"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glutReshapeFunc"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"reshape"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Function, r"glutMainLoop"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
 ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CPresentationmanager,
                syntax: concat!(
                    r"/* ",
                    output_for_hello_world!(),
                    r#" for C with PresentationManager / OS/2 2.11  */

#define INCL_WIN

#include <os2.h>

int main( void )
{
   HMQ   hmq;

   hmq = WinCreateMsgQueue( 0, 0 );

   WinMessageBox( HWND_DESKTOP, HWND_DESKTOP, (PSZ)""#,
                    output_for_hello_world!(),
                    r#"",
      (PSZ)"", 0, MB_OK );

   WinDestroyMsgQueue( hmq );

   return 0;
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"/* ",
                            output_for_hello_world!(),
                            r" for C with PresentationManager / OS/2 2.11  */"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Keyword, r"#define"),
                    (
                        TokenKind::Plain,
                        r" INCL_WIN

",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<os2.h>"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"main"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Type, r"void"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
   HMQ   hmq",
                    ),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"

   hmq ",
                    ),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"WinCreateMsgQueue"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"

   ",
                    ),
                    (TokenKind::Function, r"WinMessageBox"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r" HWND_DESKTOP"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" HWND_DESKTOP"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"PSZ"),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Punctuation, r","),
                    (
                        TokenKind::Plain,
                        r"
      ",
                    ),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"PSZ"),
                    (TokenKind::Punctuation, r")"),
                    (TokenKind::String, r#""""#),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" MB_OK "),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"

   ",
                    ),
                    (TokenKind::Function, r"WinDestroyMsgQueue"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r" hmq "),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"

   ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CWindows,
                syntax: concat!(
                    r"/* ",
                    output_for_hello_world!(),
                    r#" in C for MS-Windows */

#include <windows.h>

int PASCAL WinMain(HINSTANCE hInstance,
  HINSTANCE hPrevInstance, LPSTR CmdLine, int Show)
{
  MessageBox(GetActiveWindow(), ""#,
                    output_for_hello_world!(),
                    r#"", "Hello Windows World", MB_OK);
  return 0;
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"/* ",
                            output_for_hello_world!(),
                            r" in C for MS-Windows */"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<windows.h>"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" PASCAL "),
                    (TokenKind::Function, r"WinMain"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Type, r"HINSTANCE"),
                    (TokenKind::Plain, r" hInstance"),
                    (TokenKind::Punctuation, r","),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Type, r"HINSTANCE"),
                    (TokenKind::Plain, r" hPrevInstance"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Type, r"LPSTR"),
                    (TokenKind::Plain, r" CmdLine"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" Show"),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Function, r"MessageBox"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Function, r"GetActiveWindow"),
                    (TokenKind::Punctuation, r"(),"),
                    (TokenKind::Plain, r" "),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r#""Hello Windows World""#),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" MB_OK"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CX11Athena,
                syntax: concat!(
                    r"/* ",
                    output_for_hello_world!(),
                    r#" in C with X11 using Athena widgets */

#include <X11/Intrinsic.h>
#include <X11/StringDefs.h>
#include <X11/Xaw/Label.h>

main(int argc,char **argv)
{
  XtAppContext app_context;
  Widget toplevel,hello;

  toplevel = XtVaAppInitialize(&app_context,"XHello",NULL,0,
    &argc,argv,NULL,NULL);
  hello = XtVaCreateManagedWidget(""#,
                    output_for_hello_world!(),
                    r#"",labelWidgetClass,
    toplevel,(void*)0);

  XtRealizeWidget(toplevel);

  XtAppMainLoop(app_context);
  return 0;
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"/* ",
                            output_for_hello_world!(),
                            r" in C with X11 using Athena widgets */"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<X11/Intrinsic.h>"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<X11/StringDefs.h>"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<X11/Xaw/Label.h>"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Function, r"main"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" argc"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Type, r"char"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"**"),
                    (TokenKind::Plain, r"argv"),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
  XtAppContext app_context",
                    ),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
  Widget toplevel",
                    ),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r"hello"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"

  toplevel ",
                    ),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"XtVaAppInitialize"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Keyword, r"&"),
                    (TokenKind::Plain, r"app_context"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::String, r#""XHello""#),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r"NULL"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r","),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Keyword, r"&"),
                    (TokenKind::Plain, r"argc"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r"argv"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r"NULL"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r"NULL"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
  hello ",
                    ),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"XtVaCreateManagedWidget"),
                    (TokenKind::Punctuation, r"("),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r"labelWidgetClass"),
                    (TokenKind::Punctuation, r","),
                    (
                        TokenKind::Plain,
                        r"
    toplevel",
                    ),
                    (TokenKind::Punctuation, r",("),
                    (TokenKind::Type, r"void"),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Punctuation, r")"),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"

  ",
                    ),
                    (TokenKind::Function, r"XtRealizeWidget"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"toplevel"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"

  ",
                    ),
                    (TokenKind::Function, r"XtAppMainLoop"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"app_context"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r";"),
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
