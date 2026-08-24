use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;

use crate::core::App;
use crate::view::east::area as east_area;
use crate::view::help::help;
use crate::view::main::area as main_area;
use crate::view::north::area as north_area;
use crate::view::r#type::Hits;
use crate::view::south::area as south_area;
use crate::view::west::area as west_area;

pub fn draw(f: &mut Frame, app: &mut App) {
    let size = f.area();
    let mut hits = Hits::default();

    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(6),
            Constraint::Length(1),
        ])
        .split(size);

    north_area(f, v_chunks[0], app);

    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(20),
            Constraint::Length(3),
        ])
        .split(v_chunks[1]);

    west_area(f, h_chunks[0], app);
    main_area(f, h_chunks[1], app, &mut hits);
    east_area(f, h_chunks[2]);

    south_area(f, v_chunks[2], app);

    if app.show_help {
        help(f, size);
    }

    app.hits = hits;
}
