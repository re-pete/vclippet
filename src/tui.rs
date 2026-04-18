use std::error::Error;

use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers, read};
use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect}, symbols::border, text::Line, widgets::{Block, Borders, Paragraph}};

const QUIT_KEY: KeyEvent = KeyEvent::new(KeyCode::Char('q'),KeyModifiers::CONTROL);

struct ViewState {
    selected_panel: u8,
    panel_strings: [String; 4],
    pressed_key: Option<KeyCode>
}


struct UILayout {

}

impl UILayout {
    fn render(&self, frame: & mut Frame, state: &mut ViewState) {
        
        let instructions = Line::from("Press 1-4 to swap panes, type to type, ctrl q to quit");
        let instruction_block = Block::bordered().title(instructions.centered()).border_set(border::THICK);

        // Outer layout, has a small top rect and then the rest
        let mut outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(5),Constraint::Fill(1)])
            .split(frame.area());


        // inner vert layout, has two equal sized vertical splits
        let mut inner_vert_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50),Constraint::Percentage(50)])
            .split(outer_layout[1]);

        // top split itself has a left and right
        let mut inner_top_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50),Constraint::Percentage(50)])
            .split(inner_vert_layout[0]);

        // bottom split itself has a left and right
        let mut inner_btm_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50),Constraint::Percentage(50)])
            .split(inner_vert_layout[1]);

        // I hate this syntax so much
        if let Some(KeyCode::Char(c)) = state.pressed_key {
            state.panel_strings[state.selected_panel as usize] = format!("{}{}",state.panel_strings[state.selected_panel as usize], c);
        }


        // Top bar
        frame.render_widget(instruction_block, outer_layout[0]);

        // Top left
        frame.render_widget(Paragraph::new(state.panel_strings[0].clone()).block(Block::new().borders(Borders::ALL).title_top("Panel 1")), inner_top_layout[0]);
        // Top right
        frame.render_widget(Paragraph::new(state.panel_strings[1].clone()).block(Block::new().borders(Borders::ALL).title_top("Panel 2")), inner_top_layout[1]);

        // Bottom left
        frame.render_widget(Paragraph::new(state.panel_strings[2].clone()).block(Block::new().borders(Borders::ALL).title_top("Panel 3")), inner_btm_layout[0]);
        // Bottom right
        frame.render_widget(Paragraph::new(state.panel_strings[3].clone()).block(Block::new().borders(Borders::ALL).title_top("Panel 4")), inner_btm_layout[1]);
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {

    let mut tui = ratatui::init();
    tui.clear();
    let mut vstate = ViewState {selected_panel: 0, panel_strings:  ["".to_string(),"".to_string(),"".to_string(),"".to_string(),], pressed_key: None};

    let mut ui = UILayout {};

    
    loop {
        let _ = tui.draw(|frame| {
            ui.render(frame, &mut vstate);
        });
        // Rust syntax is disgusting
        match read() {
            Ok(event::Event::Key(keypress_evt)) => {
                if keypress_evt ==  QUIT_KEY { 
                    return Ok(());
                }
                match keypress_evt.code {
                    // This syntax makes no sense and I hate it. Oh and it's non inclusive for some
                    // reason too
                    KeyCode::Char('1'..='4') => vstate.selected_panel = keypress_evt.code.as_char().unwrap().to_digit(10).unwrap() as u8 - 1,
                    KeyCode::Char(c) => vstate.pressed_key = Some(keypress_evt.code),
                    _  => {}
                }
            }
            Ok(_) => {
            }
            Err(_) => {
                todo!()
            }
        }
    }

}
