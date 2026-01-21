use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a note in the knowledge graph.
#[derive(Debug, Clone)]
pub struct Note {
    pub path: PathBuf,
    pub title: String,
}

/// A directed graph of notes and their links.
#[derive(Debug, Default)]
pub struct NoteGraph {
    graph: DiGraph<Note, ()>,
    path_index: HashMap<PathBuf, NodeIndex>,
    title_index: HashMap<String, NodeIndex>,
}

impl NoteGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a note to the graph.
    pub fn add_note(&mut self, path: PathBuf, title: String) -> NodeIndex {
        if let Some(&idx) = self.path_index.get(&path) {
            return idx;
        }

        let note = Note {
            path: path.clone(),
            title: title.clone(),
        };
        let idx = self.graph.add_node(note);
        self.path_index.insert(path, idx);
        self.title_index.insert(title.to_lowercase(), idx);
        idx
    }

    /// Add a link from one note to another.
    pub fn add_link(&mut self, from: NodeIndex, to: NodeIndex) {
        if !self.graph.contains_edge(from, to) {
            self.graph.add_edge(from, to, ());
        }
    }

    /// Find a note by its path.
    pub fn find_by_path(&self, path: &PathBuf) -> Option<NodeIndex> {
        self.path_index.get(path).copied()
    }

    /// Find a note by its title (case-insensitive).
    pub fn find_by_title(&self, title: &str) -> Option<NodeIndex> {
        self.title_index.get(&title.to_lowercase()).copied()
    }

    /// Get all backlinks (notes that link TO this note).
    pub fn backlinks(&self, idx: NodeIndex) -> Vec<&Note> {
        self.graph
            .neighbors_directed(idx, petgraph::Direction::Incoming)
            .map(|i| &self.graph[i])
            .collect()
    }

    /// Get all forward links (notes this note links TO).
    pub fn forward_links(&self, idx: NodeIndex) -> Vec<&Note> {
        self.graph
            .neighbors_directed(idx, petgraph::Direction::Outgoing)
            .map(|i| &self.graph[i])
            .collect()
    }

    /// Get a note by its index.
    pub fn get_note(&self, idx: NodeIndex) -> Option<&Note> {
        self.graph.node_weight(idx)
    }

    /// Get all notes in the graph.
    pub fn all_notes(&self) -> Vec<&Note> {
        self.graph.node_weights().collect()
    }

    /// Get the number of notes in the graph.
    pub fn note_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get the number of links in the graph.
    pub fn link_count(&self) -> usize {
        self.graph.edge_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_operations() {
        let mut graph = NoteGraph::new();

        let note_a = graph.add_note(PathBuf::from("a.md"), "Note A".to_string());
        let note_b = graph.add_note(PathBuf::from("b.md"), "Note B".to_string());
        let note_c = graph.add_note(PathBuf::from("c.md"), "Note C".to_string());

        graph.add_link(note_a, note_b);
        graph.add_link(note_c, note_b);

        let backlinks = graph.backlinks(note_b);
        assert_eq!(backlinks.len(), 2);

        let forward = graph.forward_links(note_a);
        assert_eq!(forward.len(), 1);
        assert_eq!(forward[0].title, "Note B");
    }
}
