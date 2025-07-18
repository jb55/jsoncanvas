use std::path::PathBuf;

use crate::NodeId;
use crate::{color::Color, PixelCoordinate, PixelDimension};

use super::GenericNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileNode {
    #[serde(flatten)]
    generic: GenericNode,
    file: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    subpath: Option<String>,
}

impl FileNode {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: NodeId,
        x: PixelCoordinate,
        y: PixelCoordinate,
        width: PixelDimension,
        height: PixelDimension,
        color: Option<Color>,
        file: PathBuf,
        subpath: Option<String>,
    ) -> Self {
        Self {
            generic: GenericNode::new(id, x, y, width, height, color),
            file,
            subpath,
        }
    }

    pub fn file(&self) -> &PathBuf {
        &self.file
    }

    pub fn node(&self) -> &GenericNode {
        &self.generic
    }

    pub fn subpath(&self) -> Option<&String> {
        self.subpath.as_ref()
    }
}
