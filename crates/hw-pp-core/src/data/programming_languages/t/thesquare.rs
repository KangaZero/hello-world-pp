//! Hello-world snippet for `ProgrammingLanguageName::Thesquare`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#TheSquare>
//! - <https://esolangs.org/wiki/TheSquare>

use crate::data::programming_languages::ProgrammingParadigm::TwoDimensional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Thesquare => "TheSquare",
    versions: [Thesquare => "TheSquare"],
    extensions: [],
}

const THESQUARE: &str = r"v#############@
+:DDDDDDDDDDD:[
+#;;;;;;;;;;;#
+              
+ ;;
+  >v      ;
+ JJ+      >v
+>^-+      J-
+/--+      --
v{>[<;;    --
/- +  >v;  --
+\ +;JJ+>v --
++ +>^++J- --
++ +J++++-;--
++ >^>^++->^-
++     ++-Jv<
>^     >^-+ ;
#      v-<+>[ ^
#      - ;+
#      >-[^";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Thesquare,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Thesquare,
            syntax: THESQUARE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    THESQUARE,
                    [(
                        TokenKind::Plain,
                        r"v#############@
+:DDDDDDDDDDD:[
+#;;;;;;;;;;;#
+              
+ ;;
+  >v      ;
+ JJ+      >v
+>^-+      J-
+/--+      --
v{>[<;;    --
/- +  >v;  --
+\ +;JJ+>v --
++ +>^++J- --
++ +J++++-;--
++ >^>^++->^-
++     ++-Jv<
>^     >^-+ ;
#      v-<+>[ ^
#      - ;+
#      >-[^",
                    )],
                )
            },
        }],
    };
