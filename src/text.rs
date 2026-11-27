//! Text extraction from converted documents: the text of every paragraph in
//! one language, which is what the analysis pipeline gets as input.
//!
//! This replaces CorpusTools' `ccat`, called the way `analyse_corpus` calls
//! it (`lang=<corpus lang>`, `all_paragraphs=True`, no error options):
//!
//! - Every `<p>` in the document is a paragraph, in document order. A `<p>`
//!   is kept if its language is `lang`.
//! - The text of a paragraph is its text nodes, in order, except those in
//!   an element whose language isn't `lang`. Markup (`<em>`, `<span>`, …)
//!   adds nothing: `a<em>b</em>c` is `abc`.
//! - Error markup gives the original (erroneous) text: an `error*` element's
//!   `<correct>` children are left out. `<hyph/>` is left out, so the text on
//!   either side of it is joined.
//! - Trailing whitespace is removed. A paragraph with no text, or only
//!   whitespace, is left out.
//!
//! Languages: a `<p>` is in the language of its `xml:lang`, else in the
//! language of the document (the root's `xml:lang`), *not* of an enclosing
//! `<p>`. That is how the converter writes them: it detects the language of
//! each paragraph and sets `xml:lang` on those that aren't in the document
//! language, also on nested ones (an outer `<p xml:lang="nob">` with inner
//! `<p>`s, some `xml:lang="nob"` and some without, which are sma). ccat
//! does the same. Inside a paragraph, an element without `xml:lang` is in
//! the language of its parent.
//!
//! # Differences from ccat (on purpose)
//!
//! - **Nested paragraphs** (`<p>…<list><p>…</p></list>…</p>`, which occur in
//!   converted HTML): ccat prints the inner paragraph twice, once inside the
//!   outer one and once on its own. Here, the outer paragraph's text leaves
//!   out its inner paragraphs, and each inner paragraph is a paragraph of its
//!   own (after the outer one).
//! - **Languages inside a paragraph**: ccat gives every element without
//!   `xml:lang` the document's language, also inside a paragraph. So for an
//!   sme corpus, ccat leaves out the `b` of `<p xml:lang="sme">a<em>b</em></p>`
//!   in an nob document, and includes the `y` of
//!   `<span xml:lang="nob">x<em>y</em></span>`. No converted file has either
//!   (2026-11-25).
//! - **Whitespace**: ccat adds the paragraph's *tail* (the text after
//!   `</p>`, usually a newline) to the paragraph, and then strips trailing
//!   whitespace from that tail only. So ccat keeps the paragraph's own
//!   trailing whitespace (`"text  ¶"`), and makes an empty paragraph (` ¶`)
//!   of an empty `<p/>` followed by a newline. Here the tail isn't part of the
//!   paragraph, trailing whitespace is removed, and empty paragraphs are left
//!   out.
//! - XML comments and processing instructions give no text (lxml-based ccat
//!   would print a comment's text). Converted files have none.
//!
//! `tools/textdiff.py` in cloudanalysis compares this with ccat over whole
//! corpora.

use roxmltree::{Document, Node, NodeType};

const XML_NS: &str = "http://www.w3.org/XML/1998/namespace";

/// One paragraph of text, in the requested language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph {
    /// The text, without the ` ¶` that ccat ends a paragraph with. It may
    /// contain newlines.
    pub text: String,
    /// The `type` attribute of the `<p>` (`title`, `listitem`, `text`,
    /// `tablecell`, …), if it has one.
    pub kind: Option<String>,
}

fn is_p(node: &Node) -> bool {
    node.is_element() && node.tag_name().name() == "p"
}

/// Collect the text nodes under `node` (whose language is `node_lang`) into
/// `pieces`, following the rules in the module documentation.
fn collect<'a>(node: Node<'a, '_>, node_lang: Option<&str>, lang: &str, pieces: &mut Vec<&'a str>) {
    let in_error = node.tag_name().name().starts_with("error");
    for child in node.children() {
        match child.node_type() {
            NodeType::Text => {
                if node_lang == Some(lang) {
                    pieces.push(child.text().unwrap_or_default());
                }
            }
            NodeType::Element => {
                let name = child.tag_name().name();
                if name == "p" || name == "hyph" || (in_error && name == "correct") {
                    continue;
                }
                let child_lang = child.attribute((XML_NS, "lang")).or(node_lang);
                collect(child, child_lang, lang, pieces);
            }
            NodeType::Comment | NodeType::PI | NodeType::Root => {}
        }
    }
}

/// The text of `p`, if it has any (besides whitespace) in `lang`.
fn paragraph_text(p: Node, lang: &str) -> Option<String> {
    let mut pieces = Vec::new();
    collect(p, Some(lang), lang, &mut pieces);
    let mut text = pieces.concat();
    text.truncate(text.trim_end().len());
    (!text.is_empty()).then_some(text)
}

/// The paragraphs of `doc` that are in language `lang`, in document order.
pub fn paragraphs(doc: &Document, lang: &str) -> Vec<Paragraph> {
    let doc_lang = doc.root_element().attribute((XML_NS, "lang"));
    doc.root()
        .descendants()
        .filter(is_p)
        .filter(|p| p.attribute((XML_NS, "lang")).or(doc_lang) == Some(lang))
        .filter_map(|p| {
            let text = paragraph_text(p, lang)?;
            let kind = p.attribute("type").map(str::to_owned);
            Some(Paragraph { text, kind })
        })
        .collect()
}

/// The paragraphs in ccat's text format: each paragraph followed by ` ¶`
/// and a newline.
pub fn ccat_format(paragraphs: &[Paragraph]) -> String {
    let mut out = String::with_capacity(paragraphs.iter().map(|p| p.text.len() + 4).sum());
    for p in paragraphs {
        out.push_str(&p.text);
        out.push_str(" ¶\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts(xml: &str, lang: &str) -> Vec<String> {
        let doc = Document::parse(xml).unwrap();
        paragraphs(&doc, lang).into_iter().map(|p| p.text).collect()
    }

    #[test]
    fn markup_adds_nothing() {
        let xml = r#"<document xml:lang="sme"><body><p>a<em>b</em>c <span type="quote">d</span> e  </p></body></document>"#;
        assert_eq!(texts(xml, "sme"), ["abc d e"]);
    }

    #[test]
    fn other_language_paragraphs_and_spans_are_left_out() {
        let xml = r#"<document xml:lang="sme"><body>
            <p>dát<span xml:lang="nob">dette</span> ja</p>
            <p xml:lang="nob">bare norsk</p>
            <p xml:lang="sme">maid</p>
        </body></document>"#;
        assert_eq!(texts(xml, "sme"), ["dát ja", "maid"]);
        assert_eq!(texts(xml, "nob"), ["bare norsk"]);
    }

    #[test]
    fn inside_a_paragraph_language_is_inherited_from_the_parent() {
        // ccat would leave out "b" here (it inherits from the document)
        let xml = r#"<document xml:lang="nob"><body><p xml:lang="sme">a<em>b</em>c</p></body></document>"#;
        assert_eq!(texts(xml, "sme"), ["abc"]);
        let xml = r#"<document xml:lang="sme"><body><p>a<span xml:lang="nob">x<em>y</em></span>c</p></body></document>"#;
        assert_eq!(texts(xml, "sme"), ["ac"]);
    }

    #[test]
    fn a_paragraph_without_xml_lang_is_in_the_document_language() {
        // even inside a paragraph in another language (from corpus-sme)
        let xml = r#"<document xml:lang="sme"><body><p type="listitem" xml:lang="nob">Åarjelsaemiengïele <list><p type="listitem" xml:lang="nob">Barkoe</p><p type="listitem">Reaktah</p></list></p></body></document>"#;
        assert_eq!(texts(xml, "sme"), ["Reaktah"]);
        assert_eq!(texts(xml, "nob"), ["Åarjelsaemiengïele", "Barkoe"]);
    }

    #[test]
    fn no_document_language_means_no_text() {
        assert!(texts("<document><body><p>a</p></body></document>", "sme").is_empty());
    }

    #[test]
    fn nested_paragraphs_are_their_own_paragraphs() {
        let xml = r#"<document xml:lang="smn"><body><p type="listitem">ovdâ <list><p type="listitem">siskil</p></list> maŋa</p></body></document>"#;
        assert_eq!(texts(xml, "smn"), ["ovdâ  maŋa", "siskil"]);
    }

    #[test]
    fn errors_give_the_original_text() {
        let xml = r#"<document xml:lang="sme"><body><p>a <errorort>boa<correct>bo</correct></errorort> c</p></body></document>"#;
        assert_eq!(texts(xml, "sme"), ["a boa c"]);
    }

    #[test]
    fn hyph_joins() {
        let xml = r#"<document xml:lang="sme"><body><p>sát<hyph/>ni</p></body></document>"#;
        assert_eq!(texts(xml, "sme"), ["sátni"]);
    }

    #[test]
    fn empty_and_blank_paragraphs() {
        let xml = r#"<document xml:lang="sme"><body><p/><p></p><p> 
 </p><p><em/></p><p><em xml:lang="nob">x</em></p></body></document>"#;
        assert!(texts(xml, "sme").is_empty());
    }

    #[test]
    fn trailing_whitespace_is_trimmed() {
        let xml = r#"<document xml:lang="sme"><body><p> a <em>b </em> 
</p>
<p>c  </p></body></document>"#;
        assert_eq!(texts(xml, "sme"), [" a b", "c"]);
    }

    #[test]
    fn ccat_format_ends_paragraphs_with_pilcrow() {
        let ps = [
            Paragraph { text: "a".into(), kind: None },
            Paragraph { text: "b\nc".into(), kind: Some("title".into()) },
        ];
        assert_eq!(ccat_format(&ps), "a ¶\nb\nc ¶\n");
    }
}
