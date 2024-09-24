//!
//! A reference to a defined entity located at a specific path.
//!

use crate::easycrypt::path::Path;
use crate::easycrypt::syntax::reference::Reference;
use crate::easycrypt::syntax::Name;

///
/// A reference to a defined entity located at a specific path.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DefinitionLocation {
    /// Name of the variable.
    pub identifier: Name,
    /// Path.
    pub path: Path,
}

impl DefinitionLocation {
    ///
    /// Create an instance of `Reference` pointing here, relative to a location.
    ///
    pub fn reference(&self, relative_to: &Path) -> Reference {
        let path = if &self.path == relative_to {
            Path::empty()
        } else {
            self.path.clone()
        };
        Reference {
            identifier: self.identifier.clone(),
            path,
        }
    }
    ///
    /// Create an instance of `Reference` pointing here.
    ///
    pub fn reference_absolute(&self) -> Reference {
        Reference {
            identifier: self.identifier.clone(),
            path: self.path.clone(),
        }
    }
}
