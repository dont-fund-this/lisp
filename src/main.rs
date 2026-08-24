mod code;
mod core;
mod eval;
mod flex;
mod pane;
mod text;
mod tree;
mod view;

use std::cell::RefCell;
use std::io::stdout;
use std::rc::Rc;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    MouseButton, MouseEvent, MouseEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use slint::{CloseRequestResponse, Image, Timer, TimerMode};

use crate::core::{init, keys, mice, App};
use crate::view::{blit, draw};

slint::slint! {
    export component MainWindow inherits Window {
        title: "Steel Scheme Studio";
        min-width: 640px;
        min-height: 400px;
        background: #0f172a;

        in property <image> terminal_image;
        out property <int> cols: Math.max(40, Math.floor(self.width / 8px));
        out property <int> rows: Math.max(15, Math.floor(self.height / 16px));

        callback key_pressed(string);
        callback mouse_clicked(int, int);
        callback window_resized(int, int);

        init => {
            root.window_resized(root.cols, root.rows);
        }

        changed cols => {
            root.window_resized(root.cols, root.rows);
        }
        changed rows => {
            root.window_resized(root.cols, root.rows);
        }

        forward-focus: focus_scope;

        focus_scope := FocusScope {
            key-pressed(event) => {
                root.key_pressed(event.text);
                accept
            }

            TouchArea {
                width: 100%;
                height: 100%;

                clicked => {
                    root.mouse_clicked(
                        Math.floor(self.mouse-x / 8px),
                        Math.floor(self.mouse-y / 16px)
                    );
                }

                Image {
                    width: 100%;
                    height: 100%;
                    image-fit: fill;
                    source: root.terminal_image;
                }
            }
        }
    }
}

fn render_vt(app: &mut App, parser: &mut vt100::Parser, cols: u16, rows: u16, blink: bool) -> Image {
    let mut stream = Vec::new();
    {
        let backend = CrosstermBackend::new(&mut stream);
        let mut term = Terminal::new(backend).unwrap();
        let _ = term.draw(|f| draw(f, app));
    }
    parser.process(&stream);
    let pb = blit(parser.screen(), cols, rows, blink);
    Image::from_rgb8(pb)
}

fn run_term(app: Rc<RefCell<App>>) -> Result<()> {
    enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(out);
    let mut term = Terminal::new(backend)?;

    loop {
        {
            let mut a = app.borrow_mut();
            term.draw(|f| draw(f, &mut a))?;
            if a.should_quit {
                break;
            }
        }

        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(k) => {
                    let mut a = app.borrow_mut();
                    keys(&mut a, k);
                    if a.should_quit {
                        break;
                    }
                }
                Event::Mouse(m) => {
                    let mut a = app.borrow_mut();
                    mice(&mut a, m);
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(term.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    term.show_cursor()?;
    Ok(())
}

fn run_slint(app: Rc<RefCell<App>>) -> Result<()> {
    let win = MainWindow::new()?;

    let cur_w = Rc::new(RefCell::new(win.get_cols().max(40) as u16));
    let cur_h = Rc::new(RefCell::new(win.get_rows().max(15) as u16));
    let blink = Rc::new(RefCell::new(true));

    let parser = Rc::new(RefCell::new(vt100::Parser::new(
        *cur_h.borrow(),
        *cur_w.borrow(),
        0,
    )));

    win.window().on_close_requested(|| CloseRequestResponse::HideWindow);

    {
        let mut p = parser.borrow_mut();
        let mut a = app.borrow_mut();
        let img = render_vt(&mut a, &mut p, *cur_w.borrow(), *cur_h.borrow(), true);
        win.set_terminal_image(img);
    }

    let timer = Timer::default();
    {
        let w = win.as_weak();
        let parser = parser.clone();
        let app = app.clone();
        let cur_w = cur_w.clone();
        let cur_h = cur_h.clone();
        let blink = blink.clone();

        timer.start(
            TimerMode::Repeated,
            Duration::from_millis(500),
            move || {
                let b = !*blink.borrow();
                *blink.borrow_mut() = b;
                let mut p = parser.borrow_mut();
                let mut a = app.borrow_mut();
                let img = render_vt(&mut a, &mut p, *cur_w.borrow(), *cur_h.borrow(), b);
                if let Some(target_w) = w.upgrade() {
                    target_w.set_terminal_image(img);
                }
            },
        );
    }

    {
        let w = win.as_weak();
        let parser = parser.clone();
        let app = app.clone();
        let cur_w = cur_w.clone();
        let cur_h = cur_h.clone();
        let blink = blink.clone();

        win.on_window_resized(move |c, r| {
            let cols = (c as u16).max(40);
            let rows = (r as u16).max(15);
            *cur_w.borrow_mut() = cols;
            *cur_h.borrow_mut() = rows;
            *blink.borrow_mut() = true;

            *parser.borrow_mut() = vt100::Parser::new(rows, cols, 0);
            let mut p = parser.borrow_mut();
            let mut a = app.borrow_mut();
            let img = render_vt(&mut a, &mut p, cols, rows, true);
            if let Some(target_w) = w.upgrade() {
                target_w.set_terminal_image(img);
            }
        });
    }

    {
        let w = win.as_weak();
        let parser = parser.clone();
        let app = app.clone();
        let cur_w = cur_w.clone();
        let cur_h = cur_h.clone();
        let blink = blink.clone();

        win.on_key_pressed(move |key_str| {
            let ev = match key_str.as_str() {
                "\t" => Some(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)),
                "\n" | "\r" => Some(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
                "\u{8}" | "\u{7f}" => Some(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)),
                "\u{F700}" => Some(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)),
                "\u{F701}" => Some(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)),
                "\u{F702}" => Some(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE)),
                "\u{F703}" => Some(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE)),
                "\u{5}" => Some(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::CONTROL)),
                "\u{12}" => Some(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CONTROL)),
                "\u{14}" => Some(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::CONTROL)),
                "\u{17}" => Some(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::CONTROL)),
                "\u{2}" => Some(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::CONTROL)),
                "\u{11}" => Some(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL)),
                c if c.chars().count() == 1 => {
                    let ch = c.chars().next().unwrap();
                    Some(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE))
                }
                _ => None,
            };

            if let Some(e) = ev {
                *blink.borrow_mut() = true;
                {
                    let mut a = app.borrow_mut();
                    keys(&mut a, e);
                    if a.should_quit {
                        if let Some(target_w) = w.upgrade() {
                            let _ = target_w.hide();
                        }
                        return;
                    }
                }

                let mut p = parser.borrow_mut();
                let mut a = app.borrow_mut();
                let img = render_vt(&mut a, &mut p, *cur_w.borrow(), *cur_h.borrow(), true);
                if let Some(target_w) = w.upgrade() {
                    target_w.set_terminal_image(img);
                }
            }
        });
    }

    {
        let w = win.as_weak();
        let parser = parser.clone();
        let app = app.clone();
        let cur_w = cur_w.clone();
        let cur_h = cur_h.clone();
        let blink = blink.clone();

        win.on_mouse_clicked(move |col, row| {
            let max_w = *cur_w.borrow();
            let max_h = *cur_h.borrow();
            let col = (col as u16).clamp(0, max_w.saturating_sub(1));
            let row = (row as u16).clamp(0, max_h.saturating_sub(1));
            *blink.borrow_mut() = true;

            let m = MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: col,
                row,
                modifiers: KeyModifiers::NONE,
            };

            {
                let mut a = app.borrow_mut();
                mice(&mut a, m);
            }

            let mut p = parser.borrow_mut();
            let mut a = app.borrow_mut();
            let img = render_vt(&mut a, &mut p, max_w, max_h, true);
            if let Some(target_w) = w.upgrade() {
                target_w.set_terminal_image(img);
            }
        });
    }

    win.run()?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--size" || a == "-s") {
        if let Ok(exe) = std::env::current_exe() {
            if let Ok(meta) = exe.metadata() {
                let bytes = meta.len();
                let mb = bytes as f64 / 1_048_576.0;
                println!("{:.2} MB ({} bytes)", mb, bytes);
                return Ok(());
            }
        }
    }

    let app = Rc::new(RefCell::new(init()));
    let is_tui = args
        .iter()
        .any(|a| a == "--tui" || a == "--term" || a == "--cli" || a == "-t");

    if is_tui {
        run_term(app)
    } else {
        run_slint(app)
    }
}
