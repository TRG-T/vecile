use fs_extra::dir::get_size;
use std::fs::{metadata, read_dir, remove_dir_all, remove_file};

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
    pub path: String,
}

pub struct State {
    pub selected: Option<usize>,
}

impl State {
    pub fn new() -> State {
        State { selected: Some(0) }
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }
}

pub struct Files {
    pub state: State,
    pub files: Vec<File>,
}

impl Files {
    pub fn new(path: &mut String) -> Files {
        let mut files: Vec<File> = vec![];
        let paths = read_dir(&path).unwrap();
        for file in paths {
            let file_name = file
                .as_ref()
                .unwrap()
                .file_name()
                .to_string_lossy()
                .to_string();
            let file_path = path.to_owned() + file_name.as_str();
            let size = convert_size(get_size(&file_path).unwrap());
            if metadata(file.as_ref().unwrap().path()).unwrap().is_dir() {
                files.push(File {
                    name: file_name + "/",
                    is_dir: true,
                    size,
                    path: file_path.to_string(),
                })
            } else {
                files.push(File {
                    name: file_name,
                    is_dir: false,
                    size,
                    path: file_path.to_string(),
                })
            }
        }
        Files {
            state: State::new(),
            files,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected {
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
        let i = match self.state.selected {
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
    pub path: String,
    pub popup: Popup<'a>,
    pub files: Files,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            should_quit: false,
            path: String::from("./"),
            popup: Popup::new(
                "default",
                PopupType::Default,
                false,
                State::new(),
                ["default", "default"],
            ),
            files: Files::new(&mut String::from("./")),
        }
    }

    pub fn on_up(&mut self) {
        if self.popup.visible {
            self.popup.state.select(Some(0));
        } else {
            self.files.previous();
        }
    }

    pub fn on_down(&mut self) {
        if self.popup.visible {
            self.popup.state.select(Some(1));
        } else {
            self.files.next();
        }
    }

    pub fn on_right(&mut self) {}

    pub fn on_left(&mut self) {}

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            'd' => {
                self.popup = Popup::new(
                    "Delete file",
                    PopupType::DeleteFile,
                    true,
                    State::new(),
                    ["Cancel", "Delete"],
                );
            }
            'r' => {
                self.popup = Popup::new(
                    "Rename file",
                    PopupType::RenameFile,
                    true,
                    State::new(),
                    ["Cancel", "Confirm"],
                );
            }
            _ => {}
        }
    }

    pub fn on_enter(&mut self) {
        if !self.popup.visible && self.files.state.selected != None {
            let file = &self.files.files[self.files.state.selected.unwrap()];
            if file.is_dir {
                self.path.push_str(&file.name);
                self.files = Files::new(&mut self.path)
            }
        } else {
            match self.popup.state.selected {
                Some(0) => self.popup.visible = false,
                Some(1) => {
                    let file = &self.files.files[self.files.state.selected.unwrap()];
                    if file.is_dir {
                        remove_dir_all(&file.path).ok();
                    } else {
                        remove_file(&file.path).ok();
                    }
                    self.files = Files::new(&mut self.path);
                }
                _ => {}
            }
        }
    }

    pub fn on_esc(&mut self) {
        if self.path == "./" {
            return;
        };
        self.path.pop();
        let (start, _) = self.path.rsplit_once('/').unwrap();
        self.path = start.to_string();
        self.path.push('/');
        self.files = Files::new(&mut self.path)
    }
}

pub enum PopupType {
    Default,
    DeleteFile,
    RenameFile,
}

pub struct Popup<'a> {
    pub title: &'a str,
    pub choices: [&'a str; 2],
    pub popup_type: PopupType,
    pub visible: bool,
    pub state: State,
}

impl<'a> Popup<'a> {
    pub fn new(
        title: &'a str,
        popup_type: PopupType,
        visible: bool,
        state: State,
        choices: [&'a str; 2],
    ) -> Popup<'a> {
        Popup {
            title,
            popup_type,
            visible,
            state,
            choices,
        }
    }
}
