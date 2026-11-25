//! A parsed korp-mono document.
//!
//! For example:
//!
//! ```xml
//! <text title="" lang="" orig_lang="" first_name="" last_name="" nationality="" gt_domain="facts" date="0000-00-00" datefrom="00000000" dateto="00000000" timefrom="000000" timeto="235959"><sentence id="1">árvvo    árvu    N    N.Sg.Gen.Allegro    1    X    0
//! prošeavtta    prošeakta    N    N.Sg.Gen    1    →P    2
//! [[[...]]]
//! </sentence>
//! <sentence id="2"> [[[...]]] </sentence>
//! </text>
//! ```

/// An owned roxmltree::Document, where the reference in the
/// [`roxmltree::Document`] refers to the [`String`] stored in this
/// very struct. The struct is therefore self-referencing.
pub struct KorpMonoDocument<'a>(roxmltree::Document<'a>);

pub struct TextAttributes<'a> {
    pub title: Option<&'a str>,
    pub lang: Option<&'a str>,
    pub orig_lang: Option<&'a str>,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub nationality: Option<&'a str>,
    pub gt_domain: Option<&'a str>,
    pub date: Option<&'a str>,
    pub datefrom: Option<&'a str>,
    pub dateto: Option<&'a str>,
    pub timefrom: Option<&'a str>,
    pub timeto: Option<&'a str>,
}

pub struct Sentence<'a> {
    // Unsure if it can technically ever be negative, so i64. i64's
    // positive max value is large enough to hold all our sentences
    // in any text anyway.
    pub id: Option<i64>,
    pub content: &'a str,
}

#[derive(Debug, thiserror::Error)]
pub enum NewKorpMonoDocumentError {
    #[error("parse xml error: {0}")]
    ParseXml(#[from] roxmltree::Error),
}

impl<'a> KorpMonoDocument<'a> {
    pub fn new(s: &'a str) -> Result<Self, NewKorpMonoDocumentError> {
        Ok(Self(roxmltree::Document::parse(s)?))
    }

    fn root(&'_ self) -> roxmltree::Node<'a, '_> {
        self.0.root_element()
    }

    pub fn attributes(&'a self) -> TextAttributes<'a> {
        let root = self.root();
        TextAttributes {
            title: root.attribute("title"),
            lang: root.attribute("lang"),
            orig_lang: root.attribute("orig_lang"),
            first_name: root.attribute("first_name"),
            last_name: root.attribute("last_name"),
            nationality: root.attribute("nationality"),
            gt_domain: root.attribute("gt_domain"),
            date: root.attribute("date"),
            datefrom: root.attribute("datefrom"),
            dateto: root.attribute("dateto"),
            timefrom: root.attribute("timefrom"),
            timeto: root.attribute("timeto"),
        }
    }

    pub fn sentences(&'a self) -> impl Iterator<Item = Sentence<'a>> {
        self.0.descendants()
            .filter(|node| node.has_tag_name("sentence"))
            .map(|des| {
                let id = des.attribute("id").map(|id| id.parse().unwrap());
                let content = des.text().unwrap_or("");
                Sentence {
                    id,
                    content,
                }
            })
    }
}
