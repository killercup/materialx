use crate::{Error, MaterialX};
use indexmap::IndexMap;
use roxmltree::Document;
use smol_str::SmolStr;

use super::{meta::VersionError, AstError, Element};

impl<'xml> TryFrom<Document<'xml>> for MaterialX {
    type Error = Error;

    fn try_from(ast: Document) -> Result<Self, Self::Error> {
        if !ast.root_element().has_children() {
            return Err(Error::Empty);
        }

        let element = ast.root_element();
        let mut res = MaterialX {
            version: element
                .attribute("version")
                .ok_or(AstError::InvalidVersion(VersionError::NoVersion))?
                .parse()
                .map_err(AstError::InvalidVersion)?,
            colorspace: element.attribute("colorspace").map(|s| s.parse().unwrap()),
            elements: IndexMap::new(),
        };

        let mut children = IndexMap::new();
        for (index, child) in element.children().enumerate() {
            if !child.is_element() {
                continue;
            }
            if child.tag_name().name() == "include" {
                return Err(Error::IncludesNotSupported);
            }

            let child: Element = child.try_into().map_err(|e| AstError::Build {
                parent: MaterialX::NAME,
                index,
                source: Box::new(e),
            })?;
            children.insert(child.name.clone(), child);
        }
        res.elements = children;

        Ok(res)
    }
}

impl<'node, 'xml> TryFrom<roxmltree::Node<'node, 'xml>> for Element {
    type Error = AstError;

    fn try_from(node: roxmltree::Node) -> Result<Self, Self::Error> {
        let tag = node.tag_name().name().into();
        let name: SmolStr = node
            .attribute("name")
            .ok_or_else(|| AstError::NoName)?
            .into();

        let mut children = IndexMap::new();
        for (index, child) in node.children().enumerate() {
            if !child.is_element() {
                continue;
            }
            let child_element: Element = child.try_into().map_err(|e| AstError::Build {
                parent: name.clone(),
                index,
                source: Box::new(e),
            })?;
            children.insert(child_element.name.clone(), child_element);
        }

        Ok(Element {
            tag,
            name,
            attributes: node
                .attributes()
                .map(|a| (a.name().into(), a.value().into()))
                .collect(),
            children,
        })
    }
}
