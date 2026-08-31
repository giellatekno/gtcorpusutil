//! A parsed converted xml document.
//!
//! For example:
//!
//! ```xml
//! <document xml:lang="sme" id="no_id">
//!     <header>
//!         <title>NAV - Bearaš - Bearaš</title>
//!         <genre code="facta"/>
//!         <author>
//!             <unknown/>
//!         </author>
//!         <year></year>
//!         <translator>
//!             <unknown/>
//!         </translator>
//!         <translated_from xml:lang="nob"/>
//!             <publChannel>
//!                 <publication>
//!                     <publisher>NAV</publisher>
//!                 </publication>
//!             </publChannel>
//!         <wordcount>66</wordcount>
//!         <conversion_status type="standard"/>
//!         <availability>
//!             <free/>
//!         </availability>
//!         <submitter name="Børre Gaup" email="borre.gaup@samediggi.no"/>
//!         <multilingual>
//!             <language xml:lang="sme"/>
//!             <language xml:lang="nob"/>
//!         </multilingual>
//!         <orthography>something</orthography>
//!         <origFileName>http://www.nav.no/Samisk/Familie</origFileName><metadata><uncomplete/></metadata><version>XSLtemplate $Revision: 31551 $; file-specific xsl  Revision; common.xsl  $Revision$; </version></header>
//! <body>
//! <p type="title">Bearaš</p>
//! <p>Dáppe leat dieđut vuoigatvuođaid ja geatnegasvuođaid birra bearraša hárrái.</p>
//! <p xml:lang="nob">Váhnenruhta riegádeami oktavuođas (Foreldrepenger ved fødsel)</p>
//! <p>Váhnenruhta adopšuvnna oktavuođas (Foreldrepenger ved adopsjon)</p>
//! <p xml:lang="nob">Váhnenruhta áhččái riegádahttimis ja adopšuvnnas (Foreldrepenger til far ved fødsel og adopsjon)</p>
//! <p xml:lang="nob">Áhččivuohta ja váhnenovddasvástádus (Farskap og foreldreansvar)</p>
//! <p>Mánnáoadju (Barnetrygd)</p>
//! <p>Reaidadoarjja unnamánášváhnemiidda (Kontantstøtte)</p>
//! <p>Máksu ovdagihtii (Bidragsforskudd)</p>
//! <p>Nuppástusdoarjja (Overgangsstønad)</p>
//! <p>Mánáidgeahččama doarjja (Barnetilsyn)</p>
//! <p>Oahppodoarjja (Utdanningsstønad)</p>
//! <p xml:lang="nob">Fárrendoarjja (Tilskudd til flytting)</p>
//! <p xml:lang="nob">Doarjja ovttaskas eadnái/áhččái (Stønad til enslig mor/far)</p>
//! </body></document>
//! ```

/// An owned roxmltree::Document, where the reference in the
/// [`roxmltree::Document`] refers to the [`String`] stored in this very
/// struct. The struct is therefore self-referencing.
pub struct ConvertedDocument<'a>(roxmltree::Document<'a>);

impl<'a> ConvertedDocument<'a> {
    pub fn new(s: &'a str) -> Result<Self, roxmltree::Error> {
        Ok(Self(roxmltree::Document::parse(s)?))
    }

    pub fn body(&'a self) -> Option<roxmltree::Node<'a, 'a>> {
        self.root()
            .descendants()
            .find(|node| node.tag_name().name() == "body")
    }

    /// Return the contents of the `<body>` tag of the converted document,
    /// stripped of html tags, as text. This is essentially what `ccat` does.
    pub fn body_text(&self) -> Option<&str> {
        self.body().and_then(|node| node.text())
    }

    /// Return the text contents of xpath `/header/title`.
    pub fn title(&self) -> Option<&str> {
        self.header_descendants()?
            .find(tag_name_equals("title"))?
            .text()
    }

    /// Return the attribute contents of xpath `/header/genre[@code]`.
    pub fn genre(&self) -> Option<&str> {
        self.header_descendants()?
            .find(tag_name_equals("genre"))?
            .attribute("code")
    }

    /// Return the text contents of xpath `/header/orthography`.
    pub fn orthography(&self) -> Option<&str> {
        self.header_descendants()?
            .find(tag_name_equals("orthography"))?
            .text()
    }

    /// Return the text contents of xpath `header/year`.
    pub fn year(&self) -> Option<&str> {
        self.header_descendants()?
            .find(tag_name_equals("year"))?
            .text()
    }

    ///// Return the author
    //pub fn author(&self) -> Option<&str> {
    //    self.header_descendants()?
    //        .find(tag_name_equals("author"))?
    //}

    fn root(&'_ self) -> roxmltree::Node<'a, '_> {
        self.0.root_element()
    }

    fn header_descendants(&self) -> Option<roxmltree::Descendants<'_, '_>> {
        let header = self
            .root()
            .descendants()
            .find(|node| node.tag_name().name() == "header")?;
        Some(header.descendants())
    }
}

// helper to use in with `Node::descendants().find()` to find the node whose
// tag name is `tag_name`
fn tag_name_equals(tag_name: &str) -> impl FnMut(&roxmltree::Node<'_, '_>) -> bool {
    move |node| node.tag_name().name() == tag_name
}

fn rewrite(html: &str) -> String {
    use lol_html::html_content::{ContentType, Element};
    use lol_html::{RewriteStrSettings, element};

    let settings = RewriteStrSettings {
        element_content_handlers: vec![element!("*", |el: &mut Element| {
            // Insert the paragraph symbol before the ending </p> tag
            if el.tag_name() == "p" {
                // <p> tags can have content, so it has end_tag_handlers
                let handlers = el.end_tag_handlers().unwrap();
                handlers.push(Box::new(|end| {
                    end.before(" ¶", ContentType::Text);
                    Ok(())
                }));
            }

            // We just want the text content of the elements, so this
            // function is perfect!
            el.remove_and_keep_content();
            Ok(())
        })],
        ..RewriteStrSettings::new()
    };

    lol_html::rewrite_str(html, settings).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::converted_document::ConvertedDocument;

    const DOCUMENT: &'static str = r#"<document xml:lang="sme" id="no_id">
    <header>
        <title>NAV - Bearaš - Bearaš</title>
        <genre code="facta"/>
        <author>
            <unknown/>
        </author>
        <translator>
            <unknown/>
        </translator>
        <translated_from xml:lang="nob"/>
            <publChannel>
                <publication>
                    <publisher>NAV</publisher>
                </publication>
            </publChannel>
        <wordcount>66</wordcount>
        <conversion_status type="standard"/>
        <availability>
            <free/>
        </availability>
        <submitter name="Børre Gaup" email="borre.gaup@samediggi.no"/>
        <multilingual>
            <language xml:lang="sme"/>
            <language xml:lang="nob"/>
        </multilingual>
        <origFileName>http://www.nav.no/Samisk/Familie</origFileName><metadata><uncomplete/></metadata><version>XSLtemplate $Revision: 31551 $; file-specific xsl  Revision; common.xsl  $Revision$; </version></header>
<body>
<p type="title">Bearaš</p>
<p>Dáppe leat dieđut vuoigatvuođaid ja geatnegasvuođaid birra bearraša hárrái.</p>
<p xml:lang="nob">Váhnenruhta riegádeami oktavuođas (Foreldrepenger ved fødsel)</p>
<p>Váhnenruhta adopšuvnna oktavuođas (Foreldrepenger ved adopsjon)</p>
<p xml:lang="nob">Váhnenruhta áhččái riegádahttimis ja adopšuvnnas (Foreldrepenger til far ved fødsel og adopsjon)</p>
<p xml:lang="nob">Áhččivuohta ja váhnenovddasvástádus (Farskap og foreldreansvar)</p>
<p>Mánnáoadju (Barnetrygd)</p>
<p>Reaidadoarjja unnamánášváhnemiidda (Kontantstøtte)</p>
<p>Máksu ovdagihtii (Bidragsforskudd)</p>
<p>Nuppástusdoarjja (Overgangsstønad)</p>
<p>Mánáidgeahččama doarjja (Barnetilsyn)</p>
<p>Oahppodoarjja (Utdanningsstønad)</p>
<p xml:lang="nob">Fárrendoarjja (Tilskudd til flytting)</p>
<p xml:lang="nob">Doarjja ovttaskas eadnái/áhččái (Stønad til enslig mor/far)</p>
</body></document>"#;

    trait Peekable {
        fn peek(&self, n: usize) -> &str;
    }

    impl Peekable for str {
        fn peek(&self, n: usize) -> &str {
            match self.char_indices().take(n).last() {
                Some((i, _)) => &self[..i],
                None => self,
            }
        }
    }

    enum Comparison {
        /// They are equal
        Equal,
        /// Different. First different char index is inside.
        Difference(usize),
        /// They were equal, up to where A (leftmost) ended, but B (rightmost)
        /// went on. Think `A < B`, the index where that happened is inside.
        Longer(usize),
        /// They were equal, but then the first string went on, while the
        /// second one stopped.
        Shorter(usize),
    }

    fn compare_strings(a: &str, b: &str) -> Comparison {
        let mut ita = a.chars();
        let mut itb = b.chars();
        let mut i: usize = 0;

        use Comparison::*;
        loop {
            let a = ita.next();
            let b = itb.next();
            match (a, b) {
                (Some(cha), Some(chb)) if cha != chb => return Difference(i),
                (Some(a), Some(b)) => {
                    assert_eq!(a, b);
                    i += a.len_utf8();
                }
                (Some(_), None) => return Longer(i),
                (None, Some(_)) => return Shorter(i),
                (None, None) => return Equal,
            }
        }
    }

    #[test]
    fn xml_doc() {
        let doc: ConvertedDocument = match ConvertedDocument::new(DOCUMENT) {
            Ok(doc) => doc,
            Err(e) => panic!("{e}"),
        };

        let expected = r#"<p type="title">Bearaš</p>
<p>Dáppe leat dieđut vuoigatvuođaid ja geatnegasvuođaid birra bearraša hárrái.</p>
<p xml:lang="nob">Váhnenruhta riegádeami oktavuođas (Foreldrepenger ved fødsel)</p>
<p>Váhnenruhta adopšuvnna oktavuođas (Foreldrepenger ved adopsjon)</p>
<p xml:lang="nob">Váhnenruhta áhččái riegádahttimis ja adopšuvnnas (Foreldrepenger til far ved fødsel og adopsjon)</p>
<p xml:lang="nob">Áhččivuohta ja váhnenovddasvástádus (Farskap og foreldreansvar)</p>
<p>Mánnáoadju (Barnetrygd)</p>
<p>Reaidadoarjja unnamánášváhnemiidda (Kontantstøtte)</p>
<p>Máksu ovdagihtii (Bidragsforskudd)</p>
<p>Nuppástusdoarjja (Overgangsstønad)</p>
<p>Mánáidgeahččama doarjja (Barnetilsyn)</p>
<p>Oahppodoarjja (Utdanningsstønad)</p>
<p xml:lang="nob">Fárrendoarjja (Tilskudd til flytting)</p>
<p xml:lang="nob">Doarjja ovttaskas eadnái/áhččái (Stønad til enslig mor/far)</p>
"#;
        //        let expected = r#"Bearaš
        //Dáppe leat dieđut vuoigatvuođaid ja geatnegasvuođaid birra bearraša hárrái.
        //Váhnenruhta riegádeami oktavuođas (Foreldrepenger ved fødsel)
        //Váhnenruhta adopšuvnna oktavuođas (Foreldrepenger ved adopsjon)
        //Váhnenruhta áhččái riegádahttimis ja adopšuvnnas (Foreldrepenger til far ved fødsel og adopsjon)
        //Áhččivuohta ja váhnenovddasvástádus (Farskap og foreldreansvar)
        //Mánnáoadju (Barnetrygd)
        //Reaidadoarjja unnamánášváhnemiidda (Kontantstøtte)
        //Máksu ovdagihtii (Bidragsforskudd)
        //Nuppástusdoarjja (Overgangsstønad)
        //Mánáidgeahččama doarjja (Barnetilsyn)
        //Oahppodoarjja (Utdanningsstønad)
        //Fárrendoarjja (Tilskudd til flytting)
        //Doarjja ovttaskas eadnái/áhččái (Stønad til enslig mor/far)
        //"#;
        let actual = doc.body_text().expect("document has a <body>");

        println!(
            "expected has length = {}, actual has length = {}",
            expected.len(),
            actual.len()
        );

        match compare_strings(expected, &actual) {
            Comparison::Equal => {}
            Comparison::Difference(i) => {
                panic!(
                    "Equal up to index {i}: left continues: \"{}\", right continues: \"{}\"",
                    &expected[i..].peek(10),
                    &actual[i..].peek(10)
                );
            }
            Comparison::Longer(i) => {
                panic!(
                    "Equal up to index {i}, then right stopped. Left is {} bytes long, and continued: \"{}\"...",
                    expected.len(),
                    expected[i..].peek(10)
                );
            }
            Comparison::Shorter(i) => {
                panic!(
                    "Equal up to index {i}, then left stopped. Right is {} bytes long, and continued: \"{}\"...",
                    actual.len(),
                    actual[i..].peek(10)
                );
            }
        }

        //assert_eq!(expected, actual);
    }
}
