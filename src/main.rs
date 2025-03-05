use rocket::State;
use rocket::{Build, Rocket};
use state::Config;

#[macro_use]
extern crate rocket;

mod assets;
mod page;
mod state;
mod test_table;

#[launch]
async fn rocket() -> _ {
    let c = Config::from_env();

    let rocket = rocket::build();

    mount(rocket, c).await
}

pub type ServerState = State<state::_State>;

async fn mount(rocket: Rocket<Build>, c: Config) -> Rocket<Build> {
    let asset_routes = assets::api();
    let base_routes = page::api();
    let state = state::initial_state(c).await;
    rocket
        .mount("/_assets", asset_routes)
        .mount("/", base_routes)
        .manage(state)
}
