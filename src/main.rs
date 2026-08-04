use crab_menu::{
    app::{CrabMenu, functions::get::BoxedGetAdapter, settings::Settings}, enters::first::First, gets::files::{FileData, Files}, runs::shell::Shell, strategies::adaptive_query_limiter::AdaptiveQueryLimiter, views::list::List
};
use iced::Theme;
use std::{path::PathBuf, time::Duration};
use tracing::Level;
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    let filter = Targets::new()
        .with_target("crab_menu", Level::DEBUG)
        .with_default(Level::INFO);

    tracing_subscriber::registry()
        .with(fmt::Layer::new())
        .with(filter)
        .init();

    iced::application(
        Settings::new(
            BoxedGetAdapter(Files::new(PathBuf::from("./"), true, |data| Shell {
                shell: "sh".into(),
                args: vec!["-c".into()],
                close_it: true,
                script: format!("wl-copy {:?}", data.path),
            })),
            List {
                view_elem: |data: &FileData| {
                    data.path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned()
                },
                to_find_data: Clone::clone,
                find_data_to_string: Clone::clone,
            },
            AdaptiveQueryLimiter {
                start_target_count: 5,
                target_time: Duration::from_millis(500),
                delay_at_zero: Duration::from_millis(10000),
            },
            First,
            "0".into(),
        ),
        CrabMenu::update,
        CrabMenu::view,
    )
    .theme(Theme::Dark)
    .subscription(CrabMenu::subscription)
    .run()
    .unwrap()
}
