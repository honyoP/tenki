use std::fs;
use std::io;
use std::path::PathBuf;
use tenki_core::{NoteGraph, Scanner};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Files,
    Preview,
    Backlinks,
}

impl Pane {
    pub fn next(self) -> Self {
        match self {
            Pane::Files => Pane::Preview,
            Pane::Preview => Pane::Backlinks,
            Pane::Backlinks => Pane::Files,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Pane::Files => Pane::Backlinks,
            Pane::Preview => Pane::Files,
            Pane::Backlinks => Pane::Preview,
        }
    }
}

pub struct App {
    pub scanner: Scanner,
    pub graph: NoteGraph,
    pub files: Vec<PathBuf>,
    pub active_pane: Pane,
    pub file_list_state: usize,
    pub backlink_list_state: usize,
    pub selected_content: String,
    pub backlinks: Vec<String>,
}

impl App {
    pub fn new(notes_dir: PathBuf) -> io::Result<Self> {
        let scanner = Scanner::new(&notes_dir);
        let graph = scanner
            .scan()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        let files = scanner
            .list_files()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let mut app = Self {
            scanner,
            graph,
            files,
            active_pane: Pane::Files,
            file_list_state: 0,
            backlink_list_state: 0,
            selected_content: String::new(),
            backlinks: Vec::new(),
        };

        app.update_preview();
        Ok(app)
    }

    pub fn refresh(&mut self) -> io::Result<()> {
        self.graph = self
            .scanner
            .scan()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        self.files = self
            .scanner
            .list_files()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        self.update_preview();
        Ok(())
    }

    pub fn next_pane(&mut self) {
        self.active_pane = self.active_pane.next();
    }

    pub fn prev_pane(&mut self) {
        self.active_pane = self.active_pane.prev();
    }

    pub fn move_up(&mut self) {
        match self.active_pane {
            Pane::Files => {
                if self.file_list_state > 0 {
                    self.file_list_state -= 1;
                    self.update_preview();
                }
            }
            Pane::Backlinks => {
                if self.backlink_list_state > 0 {
                    self.backlink_list_state -= 1;
                }
            }
            Pane::Preview => {}
        }
    }

    pub fn move_down(&mut self) {
        match self.active_pane {
            Pane::Files => {
                if self.file_list_state < self.files.len().saturating_sub(1) {
                    self.file_list_state += 1;
                    self.update_preview();
                }
            }
            Pane::Backlinks => {
                if self.backlink_list_state < self.backlinks.len().saturating_sub(1) {
                    self.backlink_list_state += 1;
                }
            }
            Pane::Preview => {}
        }
    }

    pub fn select(&mut self) {
        match self.active_pane {
            Pane::Files => {
                self.update_preview();
            }
            Pane::Backlinks => {
                if let Some(backlink_title) = self.backlinks.get(self.backlink_list_state) {
                    // Find the file with this title and navigate to it
                    if let Some(idx) = self.graph.find_by_title(backlink_title) {
                        if let Some(note) = self.graph.get_note(idx) {
                            if let Some(pos) = self.files.iter().position(|f| f == &note.path) {
                                self.file_list_state = pos;
                                self.active_pane = Pane::Files;
                                self.update_preview();
                            }
                        }
                    }
                }
            }
            Pane::Preview => {}
        }
    }

    pub fn selected_file(&self) -> Option<PathBuf> {
        self.files.get(self.file_list_state).cloned()
    }

    fn update_preview(&mut self) {
        if let Some(path) = self.selected_file() {
            self.selected_content = fs::read_to_string(&path).unwrap_or_else(|_| String::new());

            // Update backlinks
            self.backlinks.clear();
            if let Some(idx) = self.graph.find_by_path(&path) {
                for note in self.graph.backlinks(idx) {
                    self.backlinks.push(note.title.clone());
                }
            }
            self.backlink_list_state = 0;
        } else {
            self.selected_content.clear();
            self.backlinks.clear();
        }
    }

    pub fn file_display_name(&self, path: &PathBuf) -> String {
        path.strip_prefix(self.scanner.root())
            .unwrap_or(path)
            .display()
            .to_string()
    }
}
