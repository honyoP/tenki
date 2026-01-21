use crate::{extract_wikilinks, NoteGraph, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Scans a directory of markdown files and builds a note graph.
pub struct Scanner {
    root: PathBuf,
}

impl Scanner {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    /// Get the root directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Scan the directory and build a graph of notes.
    pub fn scan(&self) -> Result<NoteGraph> {
        let mut graph = NoteGraph::new();
        let mut pending_links: Vec<(PathBuf, Vec<String>)> = Vec::new();

        // First pass: collect all notes
        for entry in WalkDir::new(&self.root)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md") {
                let content = fs::read_to_string(path)?;
                let title = self.extract_title(path, &content);
                let wikilinks = extract_wikilinks(&content);

                graph.add_note(path.to_path_buf(), title);
                if !wikilinks.is_empty() {
                    pending_links.push((path.to_path_buf(), wikilinks));
                }
            }
        }

        // Second pass: resolve links
        for (source_path, wikilinks) in pending_links {
            if let Some(source_idx) = graph.find_by_path(&source_path) {
                for link_target in wikilinks {
                    // Try to find target by title
                    if let Some(target_idx) = graph.find_by_title(&link_target) {
                        graph.add_link(source_idx, target_idx);
                    }
                    // TODO: Also try to match by filename without extension
                }
            }
        }

        Ok(graph)
    }

    /// List all markdown files in the directory.
    pub fn list_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.root)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md") {
                files.push(path.to_path_buf());
            }
        }

        files.sort();
        Ok(files)
    }

    /// Extract title from content or fall back to filename.
    fn extract_title(&self, path: &Path, content: &str) -> String {
        // Try to extract first H1 heading
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(title) = trimmed.strip_prefix("# ") {
                return title.to_string();
            }
        }

        // Fall back to filename without extension
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_scanner() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        fs::write(root.join("note_a.md"), "# Note A\n\nLinks to [[Note B]].")?;
        fs::write(root.join("note_b.md"), "# Note B\n\nSome content.")?;

        let scanner = Scanner::new(root);
        let graph = scanner.scan()?;

        assert_eq!(graph.note_count(), 2);
        assert_eq!(graph.link_count(), 1);

        Ok(())
    }
}
