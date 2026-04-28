use systemd::daemon::notify;
use rocket::{fairing::{Fairing, Info, Kind}, Rocket, Orbit};

pub struct Sysd;
#[rocket::async_trait]
impl Fairing for Sysd{
    fn info(&self) -> Info{
        Info{
            name: "systemd daemon",
            kind: Kind::Liftoff
        }
    }
    
    async fn on_liftoff(&self, _rocket: &Rocket<Orbit>){
        match notify(true, [("READY", "1")].iter()){
            Ok(n) => n,
            Err(e) => {
                eprintln!("{}",e);
                false
            }
        };
    }
}