use std::sync::Arc;

use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::{Semaphore, mpsc::UnboundedSender};

use crate::{
    async_template::{action::Action, config::Config},
    shared_data::{self, SharedData},
};

use super::components::Component;

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect, shared_data: &mut SharedData) -> Result<()> {
        let arr_rwlock_clone = Arc::clone(&shared_data.array);
        let render_arr = arr_rwlock_clone.read().unwrap().clone();
        let mut render_result = vec![];
        for value in render_arr {
            render_result.push(("--", value as u64));
        }

        let array_bar_chart = BarChart::default()
            .bar_width(2)
            .bar_gap(1)
            .group_gap(0)
            .label_style(Style::new().white())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(shared_data.info.clone()),
            )
            .data(&render_result);
        frame.render_widget(array_bar_chart, area);
        Ok(())
    }
}
