use serde::{Deserialize, Serialize};

use crate::NodeId;
use crate::{color::Color, PixelCoordinate, PixelDimension};

use super::GenericNode;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextNode {
    #[serde(flatten)]
    generic: GenericNode,
    text: String,
}

impl TextNode {
    pub fn new(
        id: NodeId,
        x: PixelCoordinate,
        y: PixelCoordinate,
        width: PixelDimension,
        height: PixelDimension,
        color: Option<Color>,
        text: String,
    ) -> Self {
        Self {
            generic: GenericNode::new(id, x, y, width, height, color),
            text,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn node(&self) -> &GenericNode {
        &self.generic
    }
}
