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

use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConvertedDocument {
    #[serde(rename = "@xml:lang")]
    pub lang: Option<String>,
    pub header: Header,
    //#[serde(deserialize_with = "deserialize_body")]
    pub body: HtmlBody,
}

#[derive(Deserialize)]
pub struct Header {/* TODO ... */}

pub struct HtmlBody {
    content: String,
}

impl HtmlBody {
    pub fn inner_text(&self) -> String {
        self.content.clone()
    }
}

impl<'de> serde::Deserialize<'de> for HtmlBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        println!("ENTRY POINT: HtmlBody::Deserialize::deserialize()");
        let visitor = HtmlVisitor::new();
        let s = deserializer
            .deserialize_struct("p", &["yo"], visitor)
            .unwrap();
        //let s = deserializer.deserialize_any(visitor)
        //    .expect("bruh");
        println!("BODY: {s}");
        Ok(HtmlBody { content: s })
    }
}

//use serde::de::Visitor;
//#[derive(Debug, thiserror::Error)]
//#[error("html deserializatin error")]
//pub struct HtmlDeserializerError;
//
//fn deserialize_body<'de, D, T>(d: D) -> Result<T, D::Error>
//where
//    D: serde::Deserializer<'de>
//{
//    println!("DEBUG 1");
//    let buf = String::deserialize(d)?;
//    println!("DEBUG 2");
//    unimplemented!()
//}
//
//impl serde::de::Error for HtmlDeserializerError {
//    fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
//        HtmlDeserializerError
//    }
//}
//
//pub struct HtmlDeserializer;
//
//impl<'de> serde::Deserializer<'de> for HtmlDeserializer {
//    type Error = HtmlDeserializerError;
//
//    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//    where
//        V: Visitor<'de>,
//    {
//        todo!()
//    }
//
//    serde::forward_to_deserialize_any! {
//        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
//        bytes byte_buf option unit unit_struct newtype_struct seq tuple
//        tuple_struct map struct enum identifier ignored_any
//    }
//}
//
//
//pub struct Body2 {
//    contents: MyString,
//}
//
//struct MyString(String);
//
//impl Body2 {
//    pub fn inner_text(&self) -> &str {
//        &self.contents.0
//    }
//}
//
//impl<'de> Deserialize<'de> for Body2 {
//    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//        where
//            D: serde::Deserializer<'de> {
//
//        //println!("Body2::deserialize() 1");
//        let visitor = HtmlVisitor::new();
//        //println!("Body2::deserialize() 2");
//        let contents = deserializer.deserialize_any(visitor)?;
//        //println!("Body2::deserialize() 3");
//        Ok(Body2 { contents: MyString(contents) })
//    }
//}
//
pub struct HtmlVisitor {
    output: String,
}

impl HtmlVisitor {
    pub fn new() -> Self {
        Self {
            output: String::default(),
        }
    }
}

impl<'de> serde::de::Visitor<'de> for HtmlVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "need a string, bro")
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        todo!("visit_char");
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        todo!("visit_seq")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        todo!("visit_bytes")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        todo!("visit_string");
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        todo!("visit_some");
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        todo!("visit_byte_buf");
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        //println!("HtmlVisitor::visit_map() 1");

        //let map: quick_xml::de::AttributesDeserializer = map;
        loop {
            match map.next_entry::<String, String>() {
                Ok(Some((k, v))) => {
                    // Where are the attributes?
                    println!("DEBUG: map.next_entry == Ok(Some((k, v)))");
                    println!("DEBUG: k == {k}");
                    println!("DEBUG: v == {v}");
                    self.output.push_str("<");
                    self.output.push_str(&k);
                    self.output.push_str(">");
                    self.output.push_str(&v);
                    self.output.push_str("</");
                    self.output.push_str(&k);
                    self.output.push_str(">");
                    self.output.push_str("\n");
                    //println!("Ok(Some((k, v))) - so: read {k} = {v} from map");
                }
                Ok(None) => {
                    //println!("Ok(None) => so we're done with the map now?");
                    //self.output.push_str("*** DEBUG: Ok(None) ***");
                    break;
                }
                Err(e) => {
                    //quick_xml::events::Event::Start(())
                    //match e {
                    //    quick_xml::reader::
                    //}
                    if let Some(source) = e.source() {
                        println!("source found!");
                        println!("{source}");
                    }
                    //println!("{}", e.type_id());
                    //self.output.push_str("*** DEBUG: Err(e) => e == ");
                    //self.output.push_str(&e.to_string());
                }
            }
        }

        Ok(self.output)
    }
}

//#[derive(Debug, Deserialize)]
//pub struct Body {
//    pub p: Vec<Paragraph>,
//}
//
//impl Body {
//    fn write_value(v: &Value, s: &mut String) {
//        match v {
//            Value::Em(em) => s.push_str(&em.text),
//            Value::List(inner) => {
//                for item in inner.items.iter() {
//                    Body::write_value(item, s);
//                }
//            }
//            Value::Span(string) => s.push_str(&string),
//            Value::Text(string) => s.push_str(&string),
//        }
//    }
//
//    pub fn inner_text(&self) -> String {
//        let mut out = String::new();
//
//        for p in self.p.iter() {
//            for val in p.items.iter() {
//                Body::write_value(val, &mut out);
//            }
//            out.push_str(" ¶\n");
//        }
//        out
//    }
//}
//
//#[derive(Debug, Deserialize)]
//pub struct Paragraph {
//    #[serde(rename = "@type")]
//    r#type: Option<String>,
//
//    #[serde(rename = "$value")]
//    items: Vec<Value>,
//}
//
//#[derive(Debug, Deserialize)]
//#[serde(rename_all = "snake_case")]
//pub enum Value {
//    Em(Em),
//    Span(String),
//    #[serde(rename = "$text")]
//    Text(String),
//    List(List),
//}
//
//#[derive(Debug, Deserialize)]
//pub struct Em {
//    #[serde(rename = "@xml:lang")]
//    lang: Option<String>,
//
//    #[serde(rename = "$text")]
//    text: String,
//}
//
//#[derive(Debug, Deserialize)]
//pub struct List {
//    #[serde(default)]
//    items: Vec<Value>,
//}

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
        let doc: ConvertedDocument = match quick_xml::de::from_str(DOCUMENT) {
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
        let actual = doc.body.inner_text();

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
