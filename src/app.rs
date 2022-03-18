use fs_extra::dir::get_size;
use tui::widgets::{TableState};
use std::fs;


fn convert_size(size: u64) -> String {
    let mut file_size: String;
    if size > 1000000 {
        file_size = (size / 1000000).to_string();
        file_size.push_str(" MB")
    } else if size > 1000 {
        file_size = (size / 1000).to_string();
        file_size.push_str(" kB")
    } else {
        file_size = size.to_string();
        file_size.push_str(" B")
    }
    file_size
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
            let file_path: String = default_path.to_owned() + file_name.as_str();
            let size = get_size(&file_path).unwrap();
            if fs::metadata(path.as_ref().unwrap().path())
                .unwrap()
                .is_dir()
            {
                files.push(File {
                    name: file_name + "/",
                    is_dir: true,
                    size: convert_size(size),
                    path: file_path,
                })
            } else {
                files.push(File {
                    name: file_name,
                    is_dir: false,
                    size: convert_size(size),
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
    pub default_path: String,
    pub popup: Popup<'a>,
    pub files: StatefulList,
    pub enhanced_graphics: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            default_path: String::from("./"),
            popup: Popup::new("default", PopupType::Default, false, TableState::default(), vec!["", ""]),
            files: StatefulList::new(&String::from("./")),
            enhanced_graphics,
        }
    }

    pub fn on_up(&mut self) {
        if !self.popup.visible {
            self.files.previous();
        } else {
            self.popup.state.select(Some(0));
        }
    }

    pub fn on_down(&mut self) {
        if !self.popup.visible {
            self.files.next();
        } else {
            self.popup.state.select(Some(1));
        }
    }

    pub fn on_right(&mut self) {
    }

    pub fn on_left(&mut self) {
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            },
            'd' => {
                self.popup = Popup::new("Delete file", PopupType::DeleteFile, true, TableState::default(), vec!["Delete", "Cancel"]);
            }
            _ => {}
        }
    }


    pub fn on_enter(&mut self) {
        if !self.popup.visible {
            let file = &self.files.files[self.files.state.selected().unwrap()];
            if file.is_dir {
                self.default_path.push_str(&file.name);
                self.files = StatefulList::new(&self.default_path)
            }
        } else {
            match self.popup.state.selected() {
                Some(0) => self.popup.visible = false,
                Some(1) => {                 
                    let file = &self.files.files[self.files.state.selected().unwrap()];
                    if file.is_dir {
                        fs::remove_dir_all(&file.path).ok();
                    } else {
                        fs::remove_file(&file.path).ok();
                    }
                    self.files = StatefulList::new(&self.default_path);},
                _ => {}
            }
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

pub enum PopupType {
    Default,
    DeleteFile,
}

pub struct Popup<'a> {
    pub title: &'a str,
    pub titles: Vec<&'a str>,
    pub p_type: PopupType,
    pub visible: bool,
    pub state: TableState,
}

impl<'a> Popup<'a> {
    pub fn new(title: &'a str, p_type: PopupType, visible: bool, state: TableState, titles: Vec<&'a str>) -> Popup<'a> {
        Popup { title, p_type, visible, state, titles }
    }
}
