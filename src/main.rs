use crate::app::App;

pub mod app;
pub mod config;
pub mod event;
pub mod helpers;
pub mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let config = config::load_config();
    let terminal = ratatui::init();
    let result = App::new(config).run(terminal).await;
    ratatui::restore();
    result
}
