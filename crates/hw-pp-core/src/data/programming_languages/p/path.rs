//! Hello-world snippet for `ProgrammingLanguageName::Path`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#PATH>
//! - <https://esolangs.org/wiki/PATH>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Path => "PATH",
    versions: [Path => "PATH"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Path,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Path,
            syntax: r"\/\   /\          /\  /\ 
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
            tokenized_syntax: &[(
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
        }],
    };
