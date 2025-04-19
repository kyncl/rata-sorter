use color_eyre::Result;
use ratatui::{
    prelude::*,
    style::palette::tailwind::SLATE,
    widgets::{
        Block, BorderType, Borders, Clear, HighlightSpacing, List, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Tabs,
    },
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    async_template::{action::Action, config::Config},
    frontend::generic::{Frontend, LayoutPositioning, Sets},
    shared_data::SharedData,
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
        let block = Block::default()
            .borders(Borders::all())
            .border_type(BorderType::Rounded);

        let stadndart_layout = Frontend::get_layout(
            area,
            Direction::Vertical,
            Sets::HeaderContentFooter,
            LayoutPositioning::Standart,
        );
        let header_layout = Frontend::get_layout(
            stadndart_layout[0],
            Direction::Horizontal,
            Sets::HeaderContent,
            LayoutPositioning::Inverted,
        );

        // header
        let tabs = Tabs::new(shared_data.tabs.clone())
            .block(block.clone())
            .select(shared_data.tabs_i);
        frame.render_widget(tabs, header_layout[0]);

        let state_block = Paragraph::new(shared_data.info.clone())
            .block(block.clone())
            .centered();
        frame.render_widget(state_block, header_layout[1]);

        // content
        let sort_name = "";
        frame.render_widget(
            Frontend::get_bar_chart(Some(sort_name), &shared_data.get_render_array()),
            stadndart_layout[1],
        );

        // footer
        let instructions = Line::from(vec![
            "Confirm: ".into(),
            "<Enter>".blue().bold(),
            " | ".into(),
            "Movement: ".into(),
            "<     >".blue().bold(),
            " | ".into(),
            "Quit: ".into(),
            "<Q>".blue().bold(),
            " | ".into(),
            "Cancel: ".into(),
            "<Esc>".blue().bold(),
        ]);
        let instructions = Paragraph::new(instructions).block(block.clone()).centered();
        frame.render_widget(instructions, stadndart_layout[2]);

        // popup
        let pp_area = Frontend::get_popup_area(area, 75, 50);
        let mut algorithms_list = vec![];
        for algorithm in &shared_data.sorting_algorithms {
            algorithms_list.push(algorithm.name.clone());
        }
        let algorithm_list_wid = List::new(algorithms_list)
            .block(block.clone())
            .highlight_style(Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD))
            .highlight_symbol(">>\t")
            .highlight_spacing(HighlightSpacing::WhenSelected);
        let pp_layout = Frontend::get_layout(
            pp_area,
            Direction::Vertical,
            Sets::HeaderContentFooter,
            LayoutPositioning::Standart,
        );
        let algorithm = shared_data
            .sorting_algorithms
            .get(shared_data.pp_i.selected().unwrap_or(0));
        let mut desc = "";
        if algorithm.is_some() {
            desc = &algorithm.unwrap().description;
        }

        if shared_data.show_pp {
            frame.render_widget(Clear, pp_area);
            frame.render_widget(
                Paragraph::new("Please select which algorithm you want").block(block.clone()),
                pp_layout[0],
            );
            frame.render_stateful_widget(algorithm_list_wid, pp_layout[1], &mut shared_data.pp_i);
            frame.render_widget(
                Paragraph::new(desc).block(block.clone()).centered(),
                pp_layout[2],
            );
        }

        // let vertical_scroll_state = ScrollbarState::new().position(0);
        /*         frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            pp_layout[1],
        ); */

        Ok(())
    }
}
