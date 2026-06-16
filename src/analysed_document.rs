pub struct AnalysedDocument<'a> {
    inner: roxmltree::Document<'a>,
}

impl<'a> AnalysedDocument<'a> {
    pub fn from_str(s: &'a str) -> Result<Self, roxmltree::Error> {
        Ok(Self {
            inner: roxmltree::Document::parse(s)?,
        })
    }

    fn root(&'_ self) -> roxmltree::Node<'a, '_> {
        self.inner.root_element()
    }
}
