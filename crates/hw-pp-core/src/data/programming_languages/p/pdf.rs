//! Hello-world snippet for `ProgrammingLanguageName::Pdf`.
//!
//! Imported from the Hello World Collection (`files/PDF.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/PDF.txt>
//! - <http://helloworldcollection.de/#PDF>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Pdf => "PDF",
    versions: [Pdf => "PDF"],
    extensions: [],
}

const PDF: &str = concat!(
    r"%",
    output_for_hello_world!(),
    r" in Portable Document Format (PDF)
%PDF-1.2
1 0 obj
<<
/Type /Page
/Parent 5 0 R
/Resources 3 0 R
/Contents 2 0 R
>>
endobj
2 0 obj
<<
/Length 51
>>
stream
BT
/F1 24 Tf
1 0 0 1 260 600 Tm
(",
    output_for_hello_world!(),
    r")Tj
ET
endstream
endobj
3 0 obj
<<
/ProcSet[/PDF/Text]
/Font <</F1 4 0 R >>
>>
endobj
4 0 obj
<<
/Type /Font
/Subtype /Type1
/Name /F1
/BaseFont /Arial
>>
endobj
5 0 obj
<<
/Type /Pages
/Kids [ 1 0 R ]
/Count 1
/MediaBox
[ 0 0 612 792 ]
>>
endobj
6 0 obj
<<
/Type /Catalog
/Pages 5 0 R
>>
endobj
trailer
<<
/Root 6 0 R
>>"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pdf,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pdf,
            syntax: PDF,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PDF,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"%",
                            output_for_hello_world!(),
                            r" in Portable Document Format (PDF)
%PDF-1.2
1 0 obj
<<
/Type /Page
/Parent 5 0 R
/Resources 3 0 R
/Contents 2 0 R
>>
endobj
2 0 obj
<<
/Length 51
>>
stream
BT
/F1 24 Tf
1 0 0 1 260 600 Tm
(",
                            output_for_hello_world!(),
                            r")Tj
ET
endstream
endobj
3 0 obj
<<
/ProcSet[/PDF/Text]
/Font <</F1 4 0 R >>
>>
endobj
4 0 obj
<<
/Type /Font
/Subtype /Type1
/Name /F1
/BaseFont /Arial
>>
endobj
5 0 obj
<<
/Type /Pages
/Kids [ 1 0 R ]
/Count 1
/MediaBox
[ 0 0 612 792 ]
>>
endobj
6 0 obj
<<
/Type /Catalog
/Pages 5 0 R
>>
endobj
trailer
<<
/Root 6 0 R
>>"
                        ),
                    )],
                )
            },
        }],
    };
