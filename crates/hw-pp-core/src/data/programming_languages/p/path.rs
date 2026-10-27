//! Hello-world snippet for `ProgrammingLanguageName::Path`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#PATH>
//! - <https://esolangs.org/wiki/PATH>

use crate::data::programming_languages::ProgrammingParadigm::{CellBased, TwoDimensional};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Path => "PATH",
    versions: [Path => "PATH"],
    extensions: [],
}

const PATH: &str = r"\/\   /\          /\  /\ 
+++   ++          ++  ++
+++   ++  /++++\  ++  ++  /++++\ 
++\++\++  +    +  ++  ++  +    +
++   +++  +/+++/  ++  ++  +    +
++   +++  ++      ++  ++  +    +
\/   \/\  /\+++   /\  /\  /++.+/  
                          \        \ 
/++++.+++++++++++++++++++++++++++  /  \ 
                          /\       -  -#
                          ++       .  -.
+      /  \++++\  /++++\  ++  /---\-  -+
+  /\  +  +    +  +    +  +.  -   .-  -}
+  +.  +  +    +  +       +-  -   +-  
.  +}  +  +    +  +       +-  -   +-  -.
\.+/\++/  \++++/  +       +\  /-.+/-  --
           \      /    \.{/   \    /  \/";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Path,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Path,
            syntax: PATH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PATH,
                    [(
                        TokenKind::Plain,
                        r"\/\   /\          /\  /\ 
+++   ++          ++  ++
+++   ++  /++++\  ++  ++  /++++\ 
++\++\++  +    +  ++  ++  +    +
++   +++  +/+++/  ++  ++  +    +
++   +++  ++      ++  ++  +    +
\/   \/\  /\+++   /\  /\  /++.+/  
                          \        \ 
/++++.+++++++++++++++++++++++++++  /  \ 
                          /\       -  -#
                          ++       .  -.
+      /  \++++\  /++++\  ++  /---\-  -+
+  /\  +  +    +  +    +  +.  -   .-  -}
+  +.  +  +    +  +       +-  -   +-  
.  +}  +  +    +  +       +-  -   +-  -.
\.+/\++/  \++++/  +       +\  /-.+/-  --
           \      /    \.{/   \    /  \/",
                    )],
                )
            },
        }],
    };
