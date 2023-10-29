use ratatui::prelude::{Rect, Constraint, Layout, Direction, Margin};

pub struct AppLayout {
    pub main_layout: Rect,
    pub grid_panel: Rect,
    pub config_panel: Rect,
    pub console_panel: Rect,
    pub bottom_panel: Rect,
    pub grid_constraints: Vec<Constraint>,
    grid_cell_width: usize,
    grid_cell_height: usize,
}

impl AppLayout {
    pub fn width(&self) -> usize {
        self.grid_cell_width
    }

    pub fn height(&self) -> usize {
        self.grid_cell_height
    }

    pub fn generate(terminal_rect: Rect) -> Self {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(3)])
            .split(terminal_rect.inner(&Margin::new(1, 1)));

        let bottom_panel = main_layout[1];

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(main_layout[0]);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(main_layout[1]);

        let grid_width = Self::get_grid_width(&main_layout[0]);
        let grid_height = Self::get_grid_height(&main_layout[0]);
        let constrains = std::iter::repeat(Constraint::Length(2))
            .take(grid_width as usize)
            .collect::<Vec<Constraint>>();

        Self {
            main_layout: terminal_rect,
            grid_panel: main_layout[0],
            config_panel: right_layout[0],
            console_panel: right_layout[1],
            bottom_panel,
            grid_constraints: constrains,
            grid_cell_width: grid_width,
            grid_cell_height: grid_height,
        }
    }

    fn get_grid_width(grid_panel: &Rect) -> usize {
        ((grid_panel.width - 2) / 2) as usize
    }

    fn get_grid_height(grid_panel: &Rect) -> usize {
        (grid_panel.height - 2) as usize
    }
}
