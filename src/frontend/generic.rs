use ratatui::{layout::Flex, prelude::*, widgets::*};
use std::rc::Rc;

pub enum Sets {
    Content,
    HeaderContent,
    HeaderContentFooter,
}
pub enum LayoutPositioning {
    Standart,
    Inverted,
}

pub struct Frontend {}
impl Frontend {
    pub fn get_layout(
        area: Rect,
        direction: Direction,
        sets: Sets,
        aligment: LayoutPositioning,
    ) -> Rc<[Rect]> {
        let lt: Vec<Constraint>;
        match sets {
            Sets::Content => {
                lt = vec![Constraint::Min(1)];
            }
            Sets::HeaderContent => match aligment {
                LayoutPositioning::Standart => {
                    lt = vec![Constraint::Percentage(15), Constraint::Min(1)];
                }
                LayoutPositioning::Inverted => {
                    lt = vec![Constraint::Min(1), Constraint::Percentage(15)];
                }
            },
            Sets::HeaderContentFooter => {
                lt = vec![
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ];
            }
        }
        Layout::default()
            .direction(direction)
            .constraints(lt)
            .split(area)
    }
    pub fn get_popup() {
        todo!()
    }

    pub fn get_bar_chart<'a, T>(title: Option<T>, data_set: &'a Vec<(String, u64)>) -> BarChart<'a>
    where
        T: Into<String> + Clone,
    {
        let new_data_set: Vec<(&str, u64)> =
            data_set.iter().map(|(s, num)| (s.as_str(), *num)).collect();

        let set_title: String = title.clone().map(|t| t.into()).unwrap_or_default();
        let block = Block::default()
            .borders(Borders::all())
            .border_type(BorderType::Rounded);

        BarChart::default()
            .bar_width(2)
            .bar_gap(1)
            .group_gap(0)
            .label_style(Style::new().white())
            .block(block.title(set_title))
            .data(&new_data_set)
    }

    pub fn get_popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }
}
