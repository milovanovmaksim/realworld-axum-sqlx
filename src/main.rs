use crate::app::infrastructure::pgsql::settings::DatabaseSettings;

mod app;
mod tests;

fn main() {
    println!("Hello, world!");
    let d = DatabaseSettings::from_yaml("./configuration/config.yaml");
    println!("{:#?}", d);
}
