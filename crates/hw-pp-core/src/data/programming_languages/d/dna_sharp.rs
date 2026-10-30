//! Hello-world snippet for `ProgrammingLanguageName::DnaSharp`.
//!
//! Imported from leachim6/hello-world (`d/DNA#.dna` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/DNA%23.dna>
//! - <https://esolangs.org/wiki/DNA-Sharp> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: DnaSharp => "DNA#",
    versions: [DnaSharp => "DNA#"],
    extensions: [Dna => "dna"],
}

const DNA_SHARP: &str = r"   AT
  T--A
 A----T
 T-----A
 T-----A
 G----C
  T--A
   GC
   CG
  C--G
 A----T
 A-----T
 T-----A
 A----T
  A--T
   GC
   AT
  C--G
 T----A
 C-----G
 T-----A
 G----C
  C--G
   CG
   AT
  A--T
 T----A
 A-----T
 A-----T
 G----C
  A--T
   GC
   TA
  G--C
 T----A
 G-----C
 C-----G
 C----G
  A--T
   GC
   TA
  G--C
 A----T
 G-----C
 A-----T
 C----G
  A--T
   CG
   GC
  A--T
 A----T
 C-----G
 A-----T
 C----G
  A--T
   CG
   GC
  A--T
 T----A
 G-----C
 A-----T
 G----C
  A--T
   CG
   GC
  A--T
 G----C
 A-----T
 A-----T
 T----A
  A--T
   TA
   AT
  T--A
 G----C
 A-----T
 A-----T
 A----T
  G--C
   AT
   AT
  G--C
 T----A
 G-----C
 A-----T
 G----C
  G--C
   AT
   TA
  T--A
 A----T
 G-----C
 G-----C
 A----T
  A--T
   TA
   AT
  T--A
 A----T
 T-----A
 G-----C
 A----T
  T--A
   TA
   AT
  G--C
 A----T
 T-----A
 A-----T
 T----A
  G--C
   AT
   TA
  T--A
 A----T
 G-----C
 G-----C
 A----T
  A--T
   AT
   AT
  T--A
 G----C
 A-----T";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::DnaSharp,
        extensions: &[FileExtension::Dna],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::DnaSharp,
            syntax: DNA_SHARP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DNA_SHARP,
                    [(
                        TokenKind::Plain,
                        r"   AT
  T--A
 A----T
 T-----A
 T-----A
 G----C
  T--A
   GC
   CG
  C--G
 A----T
 A-----T
 T-----A
 A----T
  A--T
   GC
   AT
  C--G
 T----A
 C-----G
 T-----A
 G----C
  C--G
   CG
   AT
  A--T
 T----A
 A-----T
 A-----T
 G----C
  A--T
   GC
   TA
  G--C
 T----A
 G-----C
 C-----G
 C----G
  A--T
   GC
   TA
  G--C
 A----T
 G-----C
 A-----T
 C----G
  A--T
   CG
   GC
  A--T
 A----T
 C-----G
 A-----T
 C----G
  A--T
   CG
   GC
  A--T
 T----A
 G-----C
 A-----T
 G----C
  A--T
   CG
   GC
  A--T
 G----C
 A-----T
 A-----T
 T----A
  A--T
   TA
   AT
  T--A
 G----C
 A-----T
 A-----T
 A----T
  G--C
   AT
   AT
  G--C
 T----A
 G-----C
 A-----T
 G----C
  G--C
   AT
   TA
  T--A
 A----T
 G-----C
 G-----C
 A----T
  A--T
   TA
   AT
  T--A
 A----T
 T-----A
 G-----C
 A----T
  T--A
   TA
   AT
  G--C
 A----T
 T-----A
 A-----T
 T----A
  G--C
   AT
   TA
  T--A
 A----T
 G-----C
 G-----C
 A----T
  A--T
   AT
   AT
  T--A
 G----C
 A-----T",
                    )],
                )
            },
        }],
    };
