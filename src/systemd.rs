use rocket::{fairing::{Fairing, Info, Kind}, Rocket, Orbit};

use libsystemd::daemon::{self, NotifyState};

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
        let sent = daemon::notify(true, &[NotifyState::Ready]);
        match sent{
            Ok(s) => s,
            Err(e) => {
                eprintln!("Notify failed{}",e);
                false
            }
        };
    }
}