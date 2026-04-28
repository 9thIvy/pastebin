#[macro_use] extern crate rocket;
mod paste_id;
mod janitor;
use paste_id::PasteId;
use rocket::tokio::fs::File;
use rocket::data::{Data, ToByteUnit};

#[launch]
fn rocket() -> _ {
    janitor::rm_old_files();
    rocket::build().mount("/", routes![index, retrieve, upload])
}

#[get("/")]
fn index() -> &'static str {
    "This is some text!"
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File>{
    File::open(id.file_path()).await.ok()
}

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String>{
    let id = PasteId::new();
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;
    // TODO: return full path. Wait until dns is done
    // also use uri!()
    Ok(id.to_string())
}