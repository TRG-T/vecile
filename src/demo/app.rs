use tui::widgets::ListState;
use std::fs;

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct File {
    pub name: String,
    pub is_dir: bool,
}

pub struct StatefulList {
    pub state: ListState,
    pub files: Vec<File>,
}

impl StatefulList {
    pub fn new(default_path: &String) -> StatefulList {
        let mut files: Vec<File> = vec![];
        let paths = fs::read_dir(default_path).unwrap();
        for path in paths {
            if fs::metadata(path.as_ref().unwrap().path()).unwrap().is_dir() {
                files.push(File { name: path.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap().to_owned() + "/", is_dir: true })
            } else {
                files.push( File { name: path.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap().to_owned(), is_dir: false })
            }

        }
        StatefulList {
            state: ListState::default(),
            files,
        } 
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.files.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.files.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub default_path: String,
    pub files: StatefulList,
    pub enhanced_graphics: bool,
}

impl<'a> App<'a> {
    pub fn new(default_path: &'a String, title: &'a str, enhanced_graphics: bool) -> App<'a> {

        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Files", "Tab1"]),
            default_path: String::from("."),
            files: StatefulList::new(default_path),
            enhanced_graphics,
        }
    }

    pub fn on_up(&mut self) {
        self.files.previous();
    }

    pub fn on_down(&mut self) {
        self.files.next();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
    
    pub fn on_enter(&mut self) {
        self.default_path.push('/');
        let _ = &self.files.files[self.files.state.selected().unwrap()].name.pop();
        self.default_path.push_str(&self.files.files[self.files.state.selected().unwrap()].name);
        self.files = StatefulList::new(&self.default_path)
    }

    pub fn on_esc(&mut self) {
        let (start, _last_word) = self.default_path.rsplit_once('/').unwrap();
        self.default_path = start.to_string();
        self.files = StatefulList::new(&self.default_path)
    }
}