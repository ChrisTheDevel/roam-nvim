use std::path::PathBuf;

use super::NodesDirWrapper;

pub struct NotesDirWrapperBuilder<'a> {
    nodes_dir: Option<PathBuf>,
    nodes_extension: Option<&'a str>,
}

impl<'a> NotesDirWrapperBuilder<'a> {
    /// Creates builder
    pub fn new() -> Self {
        NotesDirWrapperBuilder {
            nodes_dir: None,
            nodes_extension: Some("md"), // default extension
        }
    }

    pub fn nodes_dir(mut self, nodes_dir: PathBuf) -> Self {
        self.nodes_dir = Some(nodes_dir);
        self
    }

    /// This method is optional since a default value is provided by `new`
    pub fn nodes_extension(mut self, nodes_extension: &'a str) -> Self {
        self.nodes_extension = Some(nodes_extension);
        self
    }


    pub fn build(self) -> Result<NodesDirWrapper<'a>, NodesDirBuilderError> {
        if !self.nodes_dir.is_some() {
            return Err(NodesDirBuilderError {
                message: "Could not build backend since not all fields were intialzided".into(),
            });
        }
        // this is safe to unwrap since we've already checked if it is Some.
        let nodes_dir = self.nodes_dir.unwrap();
        if !nodes_dir.exists() {
            std::fs::create_dir_all(&nodes_dir).map_err(|_| NodesDirBuilderError {
                message: "Could not create the nodes dirs!".into(),
            })?;
        }
        Ok(NodesDirWrapper {
            nodes_dir,
            nodes_extension: self.nodes_extension.unwrap(),

        })
    }
}

#[derive(Clone, Debug)]
/// An error type for the DatabaseWrapperBuilder
pub struct NodesDirBuilderError {
    message: String,
}

impl std::fmt::Display for NodesDirBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "NodesDirBuilderError: {}", self.message)
    }
}

#[cfg(test)]
mod notes_dir_builder_tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_build_once() {
        let nodes_dir = std::env::temp_dir().join("test_build_once");
        let nodes_extension = "md";
        let notes_dir_wrapper = NotesDirWrapperBuilder::new()
            .nodes_dir(nodes_dir.clone())
            .nodes_extension(nodes_extension)
            .build();
        assert!(notes_dir_wrapper.is_ok());
        remove_dirs(&nodes_dir);
    }

    #[test]
    fn test_build_twice() {
        let nodes_dir = std::env::temp_dir().join("test_build_twice");
        let nodes_extension = "wiki";
        {
            let notes_dir_wrapper = NotesDirWrapperBuilder::new()
                .nodes_dir(nodes_dir.clone())
                .nodes_extension(nodes_extension)
                .build();
            }
        let notes_dir_wrapper = NotesDirWrapperBuilder::new()
            .nodes_dir(nodes_dir.clone())
            .nodes_extension(nodes_extension)
            .build();
        assert!(notes_dir_wrapper.is_ok());
        remove_dirs(&nodes_dir);
    }
}
