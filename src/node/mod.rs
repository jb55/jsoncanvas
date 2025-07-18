use crate::color::Color;
use crate::NodeId;
use crate::PixelCoordinate;
use crate::PixelDimension;
use serde::{Deserialize, Serialize};

mod file;
mod group;
mod link;
mod text;

pub use file::FileNode;
pub use group::{Background, BackgroundStyle, GroupNode};
pub use link::LinkNode;
pub use text::TextNode;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericNode {
    pub id: NodeId,
    pub x: PixelCoordinate,
    pub y: PixelCoordinate,
    pub width: PixelDimension,
    pub height: PixelDimension,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::color::Color>,
}

impl GenericNode {
    pub fn new(
        id: NodeId,
        x: PixelCoordinate,
        y: PixelCoordinate,
        width: PixelDimension,
        height: PixelDimension,
        color: Option<Color>,
    ) -> Self {
        Self {
            id,
            x,
            y,
            width,
            height,
            color,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Node {
    Text(TextNode),
    File(FileNode),
    Link(LinkNode),
    Group(GroupNode),
}

impl Node {
    pub fn node(&self) -> &GenericNode {
        match self {
            Node::Text(node) => node.node(),
            Node::File(node) => node.node(),
            Node::Link(node) => node.node(),
            Node::Group(node) => node.node(),
        }
    }
}

impl From<GroupNode> for Node {
    fn from(node: GroupNode) -> Self {
        Node::Group(node)
    }
}

impl From<TextNode> for Node {
    fn from(node: TextNode) -> Self {
        Node::Text(node)
    }
}

impl From<FileNode> for Node {
    fn from(node: FileNode) -> Self {
        Node::File(node)
    }
}

impl From<LinkNode> for Node {
    fn from(node: LinkNode) -> Self {
        Node::Link(node)
    }
}
