//! Hello-world snippet for `ProgrammingLanguageName::PostscriptPage`.
//!
//! Imported from leachim6/hello-world (`p/PostScript Page.ps` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PostScript%20Page.ps>
//! - <https://en.wikipedia.org/wiki/PostScript> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Concatenative, Procedural, StackBased,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Markup, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: PostscriptPage => "PostScript Page",
    versions: [PostscriptPage => "PostScript Page"],
    extensions: [],
}

const POSTSCRIPT_PAGE: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PostscriptPage,
        extensions: &[],
        categories: &[Markup],
        families: &[],
        paradigms: &[Procedural, StackBased, Concatenative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PostscriptPage,
            syntax: POSTSCRIPT_PAGE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    POSTSCRIPT_PAGE,
                    [(
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
                )
            },
        }],
    };
