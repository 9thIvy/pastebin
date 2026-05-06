use rocket::request::FromParam;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct PasteId<'a>(Cow<'a, str>);

impl<'a> PasteId<'_> {
    pub fn new() -> PasteId<'a> {
        let id = Uuid::new_v4();
        PasteId(Cow::Owned(id.to_string()))
    }

    pub fn file_path(&self) -> PathBuf {
        // todo: sync this so it and janitor always use same dir
        let root = "/tmp/upload";
        let filename = self.0.to_string();
        Path::new(root).join(filename)
    }
}

impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match Uuid::parse_str(param){
            Ok(_) => Ok(PasteId(Cow::Owned(param.to_string()))),
            Err(_) => Err("")
        }

    }
}

impl<'a> std::fmt::Display for PasteId<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}