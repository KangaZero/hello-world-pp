//! Hello-world snippet for `ProgrammingLanguageName::PostscriptPage`.
//!
//! Imported from leachim6/hello-world (`p/PostScript Page.ps` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: PostscriptPage => "PostScript Page",
    versions: [PostscriptPage => "PostScript Page"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PostscriptPage,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PostscriptPage,
            syntax: concat!(
                r"% run> gs -q postscript_page.ps
/pt {72 div} def
/y 9 def
/textdraw {/Courier findfont 12 pt scalefont setfont 8 pt y moveto show} def

72 72 scale
0 0 0 setrgbcolor

(",
                output_for_hello_world!(),
                r") textdraw
showpage
quit"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"% run> gs -q postscript_page.ps
/pt {72 div} def
/y 9 def
/textdraw {/Courier findfont 12 pt scalefont setfont 8 pt y moveto show} def

72 72 scale
0 0 0 setrgbcolor

(",
                    output_for_hello_world!(),
                    r") textdraw
showpage
quit"
                ),
            )],
        }],
    };
