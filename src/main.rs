use std::{marker::PhantomData, path::PathBuf, time::Duration};

use crab_menu::{
    app::{
        CrabMenu,
        functions::{
            get::{BoxedGet, BoxedGetAdapter},
            run::BoxedRun,
        },
        settings::Settings,
    },
    gets::files::{FileData, Files},
    runs::shell::Shell,
    strategies::adaptive_query_limiter::AdaptiveQueryLimiter,
    views::list::List,
};
use iced::Theme;
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
                close_it: true,
                script: format!("echo {:?}", data.path),
            })),
            List {
                view: |data: &FileData| data.path.file_name().unwrap().to_string_lossy().into_owned(),
                to_find_data: Clone::clone,
                find_data_to_string: Clone::clone
            },
            AdaptiveQueryLimiter {
                start_target_count: 5,
                target_time: Duration::from_millis(500),
                delay_at_zero: Duration::from_millis(1000),
            },
            "0".into()
        ),
        CrabMenu::update,
        CrabMenu::view,
    )
    .theme(Theme::Dark)
    .subscription(CrabMenu::subscription)
    .run()
    .unwrap()
}
