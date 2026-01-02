use std::{cell::RefCell, error::Error, ffi::OsStr, fmt::Display, rc::Rc};

use uuid::Uuid;
use winreg::types::ToRegValue;

pub type NormalResult = Result<(), Box<dyn Error>>;
pub type StdCommand = ContextCommandInfo<String, String, String, String>;
pub type StdCommandList = Vec<StdCommand>;
pub type KeyPath = Rc<RefCell<Box<[Box<str>]>>>;

#[derive(Debug, Default, Clone)]
pub struct ContextCommandInfo<
    T: ToRegValue + Display,
    U: ToRegValue + Display,
    V: ToRegValue + Display,
    W: AsRef<OsStr>,
> {
    id: Uuid,
    pub title: T,
    pub icon: U,
    pub command: V,
    pub folder: W,
    pub path: KeyPath,
}

impl<T: ToRegValue + Display, U: ToRegValue + Display, V: ToRegValue + Display, W: AsRef<OsStr>>
    ContextCommandInfo<T, U, V, W>
{
    pub fn new(title: T, icon: U, command: V, folder: W, path: KeyPath) -> Self {
        ContextCommandInfo {
            id: Uuid::new_v4(),
            title,
            icon,
            command,
            folder,
            path,
        }
    }
}

impl<T: ToRegValue + Display, U: ToRegValue + Display, V: ToRegValue + Display, W: AsRef<OsStr>>
    Display for ContextCommandInfo<T, U, V, W>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Title: {}, Command: {}", self.title, self.command)
    }
}

impl<T: ToRegValue + Display, U: ToRegValue + Display, V: ToRegValue + Display, W: AsRef<OsStr>>
    PartialEq for ContextCommandInfo<T, U, V, W>
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub enum KeyProps {
    #[default]
    None,
    HasExt,
}

impl<'a, T: Into<&'a str>> From<T> for KeyProps {
    fn from(value: T) -> Self {
        match value.into() {
            "HasExt" => Self::HasExt,
            _ => Self::None,
        }
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Key {
    pub name: String,
    pub path: KeyPath,
    pub properties: KeyProps,
}

impl Key {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Key {
            name: name.into(),
            ..Self::default()
        }
    }

    pub fn with_path<T: Into<Box<str>>>(mut self, path: T) -> Self {
        self.path = Rc::new(RefCell::new(
            path.into()
                .split("/")
                .map(Box::from)
                .collect::<Box<[Box<str>]>>(),
        ));
        self
    }

    pub fn with_props<T: Into<KeyProps>>(mut self, props: T) -> Self {
        self.properties = props.into();
        self
    }

    pub fn clone_path(&self) -> KeyPath {
        Rc::clone(&self.path)
    }

    pub fn with_extension<T: Into<Box<str>>>(self, new_extension: T) -> Self {
        match self.properties {
            KeyProps::None => self,
            KeyProps::HasExt => {
                self.path.borrow_mut()[0] = new_extension.into();
                self
            }
        }
    }

    pub fn deep_clone(&self) -> Self {
        Key {
            name: self.name.clone(),
            path: Rc::new(RefCell::new(self.path.borrow().clone().into())),
            properties: self.properties.clone(),
        }
    }
}

// Possible optimization: Change Box<[Key]> to Box<[Rc<Key>]> to make clones cheaper
// Although, Key objects are generally: a small string, an Rc, and an Enum. All of which should be cheap to clone
pub type Keys = Box<[Key]>;
