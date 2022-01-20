use filesize::file_real_size;
use fs_extra::dir::get_size;
use tui::widgets::{TableState};
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
    pub size: String,
    pub path: String
}

pub struct StatefulList {
    pub state: TableState,
    pub files: Vec<File>,
}

impl StatefulList {
    pub fn new(default_path: &str) -> StatefulList {
        let mut files: Vec<File> = vec![];
        let paths = fs::read_dir(default_path).unwrap();
        for path in paths {
            let file_name = path
                .as_ref()
                .unwrap()
                .file_name()
                .to_string_lossy()
                .to_string();
            let mut file_size: String;
            let file_path: String = default_path.to_owned() + file_name.as_str();
            if fs::metadata(path.as_ref().unwrap().path())
                .unwrap()
                .is_dir()
            {
                file_size = (get_size(&file_path).unwrap()).to_string();
                file_size.push_str(" B");
                files.push(File {
                    name: file_name + "/",
                    is_dir: true,
                    size: file_size,
                    path: file_path,
                })
            } else {
                file_size = (file_real_size(&file_path).unwrap()).to_string();
                file_size.push_str(" B");
                files.push(File {
                    name: file_name,
                    is_dir: false,
                    size: file_size,
                    path: file_path,
                })
            }
        }
        StatefulList {
            state: TableState::default(),
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
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Files", "Tab1"]),
            default_path: String::from("./"),
            files: StatefulList::new(&String::from("./")),
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
            },
            'd' => {
                let file = &self.files.files[self.files.state.selected().unwrap()];
                if file.is_dir {
                    fs::remove_dir_all(&file.path).ok();
                } else {
                    fs::remove_file(&file.path).ok();
                }
                self.files = StatefulList::new(&self.default_path);
            }
            _ => {}
        }
    }


    pub fn on_enter(&mut self) {
        let file = &self.files.files[self.files.state.selected().unwrap()];
        if file.is_dir {
            self.default_path.push_str(&file.name);
            self.files = StatefulList::new(&self.default_path)
        }
    }

    pub fn on_esc(&mut self) {
        if self.default_path == "./" {
            return;
        };
        self.default_path.pop();
        let (start, _) = self.default_path.rsplit_once('/').unwrap();
        self.default_path = start.to_string();
        self.default_path.push('/');
        self.files = StatefulList::new(&self.default_path)
    }
}
