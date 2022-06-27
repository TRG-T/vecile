use fs_extra::dir::get_size;
use std::{
    fs::{metadata, read_dir, remove_dir_all, remove_file, rename},
    io::Error,
};

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

pub struct State {
    pub selected: usize,
}

impl State {
    pub fn new() -> State {
        State { selected: 0 }
    }

    pub fn select(&mut self, index: usize) {
        self.selected = index;
    }
}

pub struct File {
    pub name: String,
    pub is_dir: bool,
    pub size: String,
    pub path: String,
}

impl File {
    pub fn new(name: String, is_dir: bool, size: String, path: String) -> File {
        File {
            name,
            is_dir,
            size,
            path,
        }
    }

    pub fn delete(&self) -> Result<(), Error> {
        if self.is_dir {
            remove_dir_all(&self.path)?;
        } else {
            remove_file(&self.path)?;
        }
        Ok(())
    }

    pub fn rename(&mut self, new_name: String) -> Result<(), Error> {
        rename(&self.name, &new_name)?;
        self.name = new_name;
        if self.is_dir {
            self.path.pop();
            self.name += "/"
        }
        let (start, _) = self.path.rsplit_once('/').unwrap();
        self.path = start.to_string() + "/" + self.name.as_str();
        Ok(())
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
            let mut file_name = file
                .as_ref()
                .unwrap()
                .file_name()
                .to_string_lossy()
                .to_string();
            let file_path = path.to_owned() + file_name.as_str();
            let size = convert_size(get_size(&file_path).unwrap());
            let is_dir = metadata(file.as_ref().unwrap().path()).unwrap().is_dir();
            if is_dir {
                file_name += "/";
            }

            files.push(File::new(file_name, is_dir, size, file_path.to_string()));
        }
        Files {
            state: State::new(),
            files,
        }
    }

    pub fn next(&mut self) {
        let i = if self.state.selected >= self.files.len() - 1 {
            0
        } else {
            self.state.selected + 1
        };
        self.state.select(i);
    }

    pub fn previous(&mut self) {
        if self.state.selected == 0 {
            return;
        }
        let i = if self.state.selected == self.files.len() - 1 {
            self.files.len() - 1
        } else {
            self.state.selected - 1
        };
        self.state.select(i);
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
                String::from(""),
            ),
            files: Files::new(&mut String::from("./")),
        }
    }

    pub fn on_up(&mut self) {
        if self.popup.visible {
            self.popup.state.select(0);
        } else {
            self.files.previous();
        }
    }

    pub fn on_down(&mut self) {
        if self.popup.visible {
            self.popup.state.select(1);
        } else {
            self.files.next();
        }
    }

    pub fn on_right(&mut self) {}

    pub fn on_left(&mut self) {}

    pub fn on_key(&mut self, c: char) {
        if self.popup.visible {
            self.popup.input.push(c);
        } else {
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
                        String::from(""),
                    );
                }
                'r' => {
                    self.popup = Popup::new(
                        "Rename file",
                        PopupType::RenameFile,
                        true,
                        State::new(),
                        ["Cancel", "Confirm"],
                        String::from(""),
                    );
                }
                _ => {}
            }
        }
    }

    pub fn on_enter(&mut self) -> Result<(), Error> {
        if !self.popup.visible {
            let file = &self.files.files[self.files.state.selected];
            if file.is_dir {
                self.path.push_str(&file.name);
                self.files = Files::new(&mut self.path);
            }
            Ok(())
        } else {
            if self.popup.state.selected == 0 {
                self.popup.visible = false;
                return Ok(())
            }
            let file = &mut self.files.files[self.files.state.selected];
            match self.popup.popup_type {
                PopupType::DeleteFile => {
                    file.delete()?;
                    self.files = Files::new(&mut self.path);
                }
                PopupType::RenameFile => {
                    file.rename(self.popup.input.to_string())?;
                }
                _ => {}
            }
            self.popup.visible = false;
            Ok(())
        }
    }

    pub fn on_esc(&mut self) {
        if self.path == "./" || self.popup.visible {
            return;
        };
        self.path.pop();
        let (start, _) = self.path.rsplit_once('/').unwrap();
        self.path = start.to_string();
        self.path.push('/');
        self.files = Files::new(&mut self.path)
    }

    pub fn on_backspace(&mut self) {
        if self.popup.visible {
            if let PopupType::RenameFile = self.popup.popup_type {
                self.popup.input.pop();
            }
        }
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
    pub input: String,
}

impl<'a> Popup<'a> {
    pub fn new(
        title: &'a str,
        popup_type: PopupType,
        visible: bool,
        state: State,
        choices: [&'a str; 2],
        input: String,
    ) -> Popup<'a> {
        Popup {
            title,
            popup_type,
            visible,
            state,
            choices,
            input,
        }
    }
}

impl State {}
