use std::error::Error;

use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers, read};
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, symbols::border, text::Line, widgets::{Block, Paragraph}};

static QUIT_KEY: KeyEvent = KeyEvent::new(KeyCode::Char('q'),KeyModifiers::CONTROL);

struct ViewState {
    selected_panel: u8,
    panel_strings: [String; 4],
}

pub fn run() -> Result<(), Box<dyn Error>> {

    let mut tui = ratatui::init();
    let mut vstate = ViewState {selected_panel: 1, panel_strings:  ["".to_string(),"".to_string(),"".to_string(),"".to_string(),]};

    let instructions = Line::from("Press 1-4 to swap panes, type to type, ctrl q to quit");
    let instruction_block = Block::bordered().title(instructions.centered()).border_set(border::THICK);

    // Outer layout, has a small top rect and then the rest
    let mut outer_layout : Layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(5),Constraint::Fill(1)]);


    // inner vert layout, has two equal sized vertical splits
    let mut inner_vert_layout : Layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50),Constraint::Percentage(50)]);

    let mut inner_top_layout : Layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50),Constraint::Percentage(50)]);

    let mut inner_btm_layout : Layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50),Constraint::Percentage(50)]);

    
    let mut pressed_key : Option<char> = None;
    loop {
        tui.draw(|frame| {

            let outer_layout_2 = outer_layout.split(frame.area());

            k
            frame.render_widget(&instruction_block, outer_layout_2[0]);



            
            vstate.panel_strings[vstate.selected_panel as usize] = format!("{}{}",vstate.panel_strings[vstate.selected_panel as usize],pressed_key.unwrap_or(' '));
            let widget = Paragraph::new(vstate.panel_strings[vstate.selected_panel as usize].clone());
            frame.render_widget(&instruction_block, rects[0]);
            frame.render_widget(widget, rects[1]);
        });
        match read() {
            Ok(event::Event::Key(keypress_evt)) => {
                if keypress_evt == QUIT_KEY {
                    return Ok(());
                }
                else {
                    pressed_key = keypress_evt.code.as_char();
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
