///parse the xml bible into a Document
pub fn parse(bible_string: &str) -> anyhow::Result<roxmltree::Document> {
    let bible = roxmltree::Document::parse(bible_string)?;
    Ok(bible)
}


#[cfg(test)]
mod tests {
    use crate::parse::*;

    #[test]
    #[ignore]
    fn test_parse() {
        let doc = parse(
            r#"
        <verse osisID='Gen.1.1'>In the beginning God created the heavens and the earth.</verse>
              "#,
        )
        .unwrap();

        let elem: roxmltree::Node = doc
            .descendants()
            .find(|n| n.attribute("osisID") == Some("Gen.1.1"))
            .unwrap();
        assert!(elem.has_tag_name("verse"));
    }
}
