use std::{fs, path::PathBuf, time::SystemTime};
use tokio::time::{self, Duration};

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
    let paths = fs::read_dir("./")?
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
    loop{
        let mut interval = time::interval(Duration::from_hours(1));
        interval.tick().await;
    
        let paths = match find_files(){
            Ok(p) => p,
            Err(e) => panic!("in find_files() called by rm_old_files: {}",e)
        };
        
        for path in paths{
            match fs::remove_file(path){
                Ok(_) => (),
                Err(e) => eprintln!("can not remove file!\n{}",e),
            }
        }
    }
}
