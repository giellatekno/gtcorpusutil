//! A parsed analysed xml document.
//!
//! For example:
//!
//! ```xml
//! <document xml:lang="sme" id="no_id">
//!     <header>
//!         <title>Fágaplána. Sámi vuođđoskuvlaoahpaheaddjeoahppu 5-10. Luonddufága 1</title>
//!         <genre code="admin"/>
//!         <author><unknown/></author>
//!         <year>2014</year>
//!         <wordcount>726</wordcount>
//!         <conversion_status type="standard"/>
//!         <availability><free/></availability>
//!         <origFileName>
//!             http://sh.pedit.no/web/NettskoleRessurs.axd?id=1ad2b9d1-f28e-400f-9520-193253d79f95
//!         </origFileName>
//!         <metadata>
//!             <uncomplete/>
//!         </metadata>
//!         <version>
//!             XSLtemplate $Revision$; file-specific xsl  Revision; common.xsl  $Revision$;
//!         </version>
//!         <note>Source http://sh.pedit.no/web/PageND.aspx?id=166687</note>
//!     </header>
//!     <body>
//!         <dependency>
//!             <![CDATA["<FÁGAPLÁNA>" ((massive text...)) ]]>
//!         </dependency>
//!     </body>
//! </document>
//! ```

/// An owned roxmltree::Document, where the reference in the
/// [`roxmltree::Document`] refers to the [`String`] stored in this
/// very struct. The struct is therefore self-referencing.
pub struct AnalysedDocument<'a>(roxmltree::Document<'a>);

#[derive(Debug, thiserror::Error)]
pub enum NewAnalysedDocumentError {
    #[error("parse xml error: {0}")]
    ParseXml(#[from] roxmltree::Error),
}

impl<'a> AnalysedDocument<'a> {
    pub fn new(s: &'a str) -> Result<Self, NewAnalysedDocumentError> {
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

    /// Return the text contents of xpath `header/year`.
    pub fn year(&self) -> Option<&str> {
        self.header_descendants()?
            .find(crate::converted_document::tag_name_equals("year"))?
            .text()
    }

    /// Return the text contents of xpath `header/title`.
    pub fn title(&self) -> Option<&str> {
        self.header_descendants()?
            .find(crate::converted_document::tag_name_equals("title"))?
            .text()
    }

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
