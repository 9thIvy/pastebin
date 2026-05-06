use rocket::{fairing::{Fairing, Info, Kind}, Rocket, Orbit};
use std::{fs, path::PathBuf, time::SystemTime};
use tokio::time::{self, Duration};
use tokio::task;


pub struct Janitor;
#[rocket::async_trait]
impl Fairing for Janitor{
    fn info(&self) -> Info{
        Info{
            name: "Remove old files",
            kind: Kind::Liftoff
        }
    }

    async fn on_liftoff(&self, _rocket: &Rocket<Orbit>){

        match tokio::fs::create_dir("/tmp/upload").await {
            Ok(r) => r,
            Err(e) => eprintln!("{}",e),
        }
        
        task::spawn(async {
            rm_old_files().await;
        });
    }
}


fn too_old(p: &std::path::PathBuf, now: u64) -> bool {
    let metadata = match fs::metadata(p) {
        Ok(m) => m,
        Err(_) => return true,
    };

    let age_sys_time = match metadata.created() {
        Ok(a) => a,
        Err(_) => return true,
    };

    let age_seconds = match age_sys_time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(a) => a.as_secs(),
        Err(_) => return true,
    };

    if (now - age_seconds) / 3600 >= 1 {
        return true;
    }
    return false;
}

fn find_files() -> std::io::Result<Vec<std::path::PathBuf>> {
    let paths = fs::read_dir("/tmp/upload")?
        .map(|res| res.map(|p| p.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    let old_files = paths.into_iter()
        .filter(|p| too_old(p, now))
        .collect::<Vec<_>>();
    Ok(old_files)
}
pub async fn rm_old_files() {
    let mut interval = time::interval(Duration::from_hours(1));
    loop{
        interval.tick().await;

        let paths: Vec<PathBuf> = match task::spawn_blocking(||{
            match find_files(){
                Ok(p) => p,
                Err(e) => panic!("find_files error:{}",e)
            }
        }).await{
            Ok(p) => p,
            Err(e) => panic!("blocking task panicked: {}",e),
        };

        for path in paths{
            match tokio::fs::remove_file(path).await{
                Ok(_) => (),
                Err(e) => eprintln!("cant remove file{}",e)
            }
        }
    }
}
