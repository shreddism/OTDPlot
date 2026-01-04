use std::time::{Duration, Instant};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::{self},
    text::Span,
    widgets::{Axis, Block, Chart, Dataset, GraphType},
    DefaultTerminal, Frame,
};

const ARRAY_HISTORY: usize = 10;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    window: [f64; 2],
}

impl App {
    fn new() -> Self {
        Self {
            window: [-160.0, 160.0],
        }
    }

    fn run(self, mut terminal: DefaultTerminal) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let (tvx, rvx) = mpsc::channel();
        let (tax, rax) = mpsc::channel();
        let (tjx, rjx) = mpsc::channel();
        let (tsx, rsx) = mpsc::channel();
        let (tvy, rvy) = mpsc::channel();
        let (tay, ray) = mpsc::channel();
        let (tjy, rjy) = mpsc::channel();
        let (tsy, rsy) = mpsc::channel();

        thread::spawn(move || {

            let child = Command::new("otd-daemon")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

            let stdout = child.unwrap().stdout.take().expect("OUCH!");
            let reader = BufReader::new(stdout);

            thread::spawn(move || {
                for line in reader.lines()
                {
                    match line {
                        Ok(data) => { 
                            let mut identifier = data.clone();
                            if (identifier.len() > 1) {
                                identifier = identifier[..2].to_string();

                                if identifier == "xx" {
                                    tx.send(identifier).unwrap();   
                                    continue;
                                }
                                else if identifier == "vx" {
                                    tx.send(identifier).unwrap();                 
                                    tvx.send(data[2..].to_string()).unwrap();
                                }
                                else if identifier == "ax" {
                                    tx.send(identifier).unwrap();
                                    tax.send(data[2..].to_string()).unwrap();
                                }
                                else if identifier == "jx" {
                                    tx.send(identifier).unwrap();
                                    tjx.send(data[2..].to_string()).unwrap();
                                }   
                                else if identifier == "sx" {
                                    tx.send(identifier).unwrap();
                                    tsx.send(data[2..].to_string()).unwrap();
                                }
                                else if identifier == "vy" {
                                    tx.send(identifier).unwrap();                 
                                    tvy.send(data[2..].to_string()).unwrap();
                                }
                                else if identifier == "ay" {
                                    tx.send(identifier).unwrap();
                                    tay.send(data[2..].to_string()).unwrap();
                                }
                                else if identifier == "jy" {
                                    tx.send(identifier).unwrap();
                                    tjy.send(data[2..].to_string()).unwrap();
                                }   
                                else if identifier == "sy" {
                                    tx.send(identifier).unwrap();
                                    tsy.send(data[2..].to_string()).unwrap();
                                }
                                else if identifier == "dd" {
                                    tx.send(identifier).unwrap();
                                }
                                else if identifier == "ii" {
                                    tx.send(identifier).unwrap();
                                }
                                else if identifier == "tt" {
                                    tx.send(identifier).unwrap();
                            }
                            }
                        }
                    Err(_) => todo!()
                    }
                }
            });   
        });

        let mut inputsens: f64 = 1.0;
        let mut vxinfo: f64 = -0.01;
        let mut axinfo: f64 = -0.01;
        let mut jxinfo: f64 = -0.01;
        let mut sxinfo: f64 = -0.01;
        let mut vyinfo: f64 = -0.01;
        let mut ayinfo: f64 = -0.01;
        let mut jyinfo: f64 = -0.01;
        let mut syinfo: f64 = -0.01;
        let mut arrayv: [(f64, f64); ARRAY_HISTORY] = [(0.0_f64, 0.0_f64); ARRAY_HISTORY];
        let mut arraya: [(f64, f64); ARRAY_HISTORY] = [(0.0_f64, 0.0_f64); ARRAY_HISTORY];
        let mut arrayj: [(f64, f64); ARRAY_HISTORY] = [(0.0_f64, 0.0_f64); ARRAY_HISTORY];
        let mut arrays: [(f64, f64); ARRAY_HISTORY] = [(0.0_f64, 0.0_f64); ARRAY_HISTORY];
        let mut vxdiv: f64 = 1.0;
        let mut axdiv: f64 = 1.0;
        let mut jxdiv: f64 = 1.0;
        let mut sxdiv: f64 = 1.0;
        let mut vydiv: f64 = 1.0;
        let mut aydiv: f64 = 1.0;
        let mut jydiv: f64 = 1.0;
        let mut sydiv: f64 = 1.0;
        let mut zeropoint: [(f64, f64); 2] = [(0.0_f64, 0.0_f64); 2];
            
        let tick_rate = Duration::from_nanos(1);
        let mut last_tick = Instant::now();

        let mut mode: u8 = 1;

        terminal.draw(|frame| self.draw(frame, arrayv, arraya, arrayj, arrays, mode, inputsens, zeropoint))?;
                
        loop {  
            match rx.recv().unwrap() {
                val if val == "xx".to_string() => {
                    let mut index = ARRAY_HISTORY - 1;
                    
                    arrayv[0].0 = vxinfo / vxdiv;
                    arraya[0].0 = axinfo / axdiv;
                    arrayj[0].0 = jxinfo / jxdiv;
                    arrays[0].0 = sxinfo / sxdiv;
                    arrayv[0].1 = vyinfo / vydiv;
                    arraya[0].1 = ayinfo / aydiv;
                    arrayj[0].1 = jyinfo / jydiv;
                    arrays[0].1 = syinfo / sydiv;
            
                        
                    while index > 0 {

                        arrayv[index].0 = arrayv[index - 1].0;
                        arraya[index].0 = arraya[index - 1].0;
                        arrayj[index].0 = arrayj[index - 1].0;
                        arrays[index].0 = arrays[index - 1].0;

                        
                        arrayv[index].1 = arrayv[index - 1].1;
                        arraya[index].1 = arraya[index - 1].1;
                        arrayj[index].1 = arrayj[index - 1].1;
                        arrays[index].1 = arrays[index - 1].1;
                        index = index - 1;
                    }
                    last_tick = Instant::now()
                },
                val if val == "vx".to_string() => {
                    let vxr: f64 = match rvx.recv().unwrap().trim().parse() {
                        Ok(num) => num,
                        Err(_) => 0.01,
                    };
                    vxinfo = vxr;
                },
                val if val == "ax".to_string() => {
                    let axr: f64 = match rax.recv().unwrap().trim().parse() {
                        Ok(num) => num,
                        Err(_) => 0.01,
                    };
                    axinfo = axr;
                },
                val if val == "jx".to_string() => {
                    let jxr: f64 = match rjx.recv().unwrap().trim().parse() {
                        Ok(num) => num,
                        Err(_) => 0.01,
                    };
                    jxinfo = jxr;
                },
                val if val == "sx".to_string() => {
                    let sxr: f64 = match rsx.recv().unwrap().trim().parse() {
                        Ok(num) => num,
                        Err(_) => 0.01,
                    };
                    sxinfo = sxr;
                },
                val if val == "vy".to_string() => {
                    let vyr: f64 = match rvy.recv().unwrap().trim().parse() {
                        Ok(num) => num,
                        Err(_) => 0.01,
                    };
                    vyinfo = vyr;
                },
                val if val == "ay".to_string() => {
                    let ayr: f64 = match ray.recv().unwrap().trim().parse() {
                        Ok(num) => num,
                        Err(_) => 0.01,
                    };
                    ayinfo = ayr;
                },
                val if val == "jy".to_string() => {
                    let jyr: f64 = match rjy.recv().unwrap().trim().parse() {
                        Ok(num) => num,
                        Err(_) => 0.01,
                    };
                    jyinfo = jyr;
                },
                val if val == "sy".to_string() => {
                    let syr: f64 = match rsy.recv().unwrap().trim().parse() {
                        Ok(num) => num,
                        Err(_) => 0.01,
                    };
                    syinfo = syr;
                },
                val if val == "dd".to_string() => {
                    terminal.draw(|frame| self.draw(frame, arrayv, arraya, arrayj, arrays, mode, inputsens, zeropoint))?;
                    let timeout = tick_rate.saturating_sub(last_tick.elapsed());
                    if event::poll(timeout)? {
                        if let Event::Key(key) = event::read()? {
                            match key.code { 
                            KeyCode::Char('q') => {
                                let _kill = Command::new("pkill")
                                                        .arg("-f")
                                                        .arg("OpenTabletDriver.Daemon")
                                                        .output()
                                                        .unwrap();
                                return Ok(());
                            },
                            KeyCode::Left => {
                                if mode > 0 { mode = mode - 1 }
                                else { mode = 15 };
                            },
                            KeyCode::Right => {
                                if mode < 15 { mode = mode + 1 }
                                else { mode = 0 };
                            },
                            KeyCode::Up => {
                                match mode {
                                    0 => { inputsens = inputsens + 0.1 },
                                    1 => { vxdiv = vxdiv + inputsens;
                                           vydiv = vydiv + inputsens; },
                                    2 => { axdiv = axdiv + inputsens;
                                           aydiv = aydiv + inputsens; },
                                    4 => { jxdiv = jxdiv + inputsens;
                                           jydiv = jydiv + inputsens; },
                                    8 => { sxdiv = sxdiv + inputsens;
                                           sydiv = sydiv + inputsens; },
                                    _ => {},
                                }
                            },
                            KeyCode::Down => {
                                match mode {
                                    0 => { inputsens = inputsens - 0.1 },
                                    1 => { vxdiv = vxdiv - inputsens;
                                           vydiv = vydiv - inputsens; },
                                    2 => { axdiv = axdiv - inputsens;
                                           aydiv = aydiv - inputsens; },
                                    4 => { jxdiv = jxdiv - inputsens;
                                           jydiv = jydiv - inputsens; },
                                    8 => { sxdiv = sxdiv - inputsens;
                                           sydiv = sydiv - inputsens; },
                                    _ => {},
                                }
                            },
                            _ => {},
                            }
                            
                        }
                    }
                },
                val if val == "ii".to_string() => {
                    let timeout = tick_rate.saturating_sub(last_tick.elapsed());
                    if event::poll(timeout)? {
                        if let Event::Key(key) = event::read()? {
                            match key.code { 
                            KeyCode::Char('q') => {
                                let _kill = Command::new("pkill")
                                                        .arg("-f")
                                                        .arg("OpenTabletDriver.Daemon")
                                                        .output()
                                                        .unwrap();
                                return Ok(());
                            },
                            KeyCode::Left => {
                                if mode > 0 { mode = mode - 1 }
                                else { mode = 15 };
                            },
                            KeyCode::Right => {
                                if mode < 15 { mode = mode + 1 }
                                else { mode = 0 };
                            },
                            KeyCode::Up => {
                                match mode {
                                    0 => { inputsens = inputsens + 0.1 },
                                    1 => { vxdiv = vxdiv + inputsens;
                                           vydiv = vydiv + inputsens; },
                                    2 => { axdiv = axdiv + inputsens;
                                           aydiv = aydiv + inputsens; },
                                    4 => { jxdiv = jxdiv + inputsens;
                                           jydiv = jydiv + inputsens; },
                                    8 => { sxdiv = sxdiv + inputsens;
                                           sydiv = sydiv + inputsens; },
                                    _ => {},
                                }
                            },
                            KeyCode::Down => {
                                match mode {
                                    0 => { inputsens = inputsens - 0.1 },
                                    1 => { vxdiv = vxdiv + inputsens;
                                           vydiv = vydiv + inputsens; },
                                    2 => { axdiv = axdiv + inputsens;
                                           aydiv = aydiv + inputsens; },
                                    4 => { jxdiv = jxdiv + inputsens;
                                           jydiv = jydiv + inputsens; },
                                    8 => { sxdiv = sxdiv + inputsens;
                                           sydiv = sydiv + inputsens; },
                                    _ => {},
                                }
                            },
                            _ => {},
                            }
                            terminal.draw(|frame| self.draw(frame, arrayv, arraya, arrayj, arrays, mode, inputsens, zeropoint))?;
                        }
                    }   
                },
                val if val == "tt".to_string() => {
                    let mut index = ARRAY_HISTORY - 1;
                    arrayv[0].0 = 0.0;
                    arrayv[0].1 = vyinfo / vydiv;
                    arraya[0].0 = 0.0;
                    arraya[0].1 = ayinfo / aydiv;
                    arrayj[0].0 = 0.0;
                    arrayj[0].1 = jyinfo / jydiv;
                    arrays[0].0 = 0.0;
                    arrays[0].1 = syinfo / sydiv;
                        
                    while index > 0 {

                        arrayv[index].0 = index as f64 * -1.0;
                        arrayv[index].1 = arrayv[index - 1].1;
                        arraya[index].0 = index as f64 * -1.0;
                        arraya[index].1 = arraya[index - 1].1;
                        arrayj[index].0 = index as f64 * -1.0;
                        arrayj[index].1 = arrayj[index - 1].1;
                        arrays[index].0 = index as f64 * -1.0;
                        arrays[index].1 = arrays[index - 1].1;
                        index = index - 1;
                    }
                }
                _ => {},
            }
        }
    }

    fn draw(&self, frame: &mut Frame, arrayv: [(f64, f64); ARRAY_HISTORY],
                                      arraya: [(f64, f64); ARRAY_HISTORY],
                                      arrayj: [(f64, f64); ARRAY_HISTORY],
                                      arrays: [(f64, f64); ARRAY_HISTORY],
                                      mode: u8, 
                                      inputsens: f64, 
                                      zeropoint: [(f64, f64); 2]) {
        self.render_animated_chart(frame, frame.area(), arrayv, arraya, arrayj, arrays, mode, inputsens, zeropoint);
    }

    fn render_animated_chart(&self, frame: &mut Frame, area: Rect, arrayv: [(f64, f64); ARRAY_HISTORY],
                                                                   arraya: [(f64, f64); ARRAY_HISTORY],
                                                                   arrayj: [(f64, f64); ARRAY_HISTORY],
                                                                   arrays: [(f64, f64); ARRAY_HISTORY], 
                                                                   mode: u8, 
                                                                   inputsens: f64, 
                                                                   zeropoint: [(f64, f64); 2]) {
        let x_labels = vec![
            match mode {
                0 => {Span::raw(format!("Press right to change mode and start using pen. Input sensitivity can be changed here with up/down."))},
                1 => {Span::raw(format!("Velocity divisor can be changed here with up/down."))},
                2 => {Span::raw(format!("Acceleration divisor can be changed here with up/down."))},
                4 => {Span::raw(format!("Jerk divisor can be changed here with up/down."))},
                8 => {Span::raw(format!("Snap divisor can be changed here with up/down."))},
                _ => {Span::raw(format!(""))},
            }
        ];

        let datasets = vec![
            Dataset::default()
                .name("zeropoint")
                .marker(symbols::Marker::Braille)
                .style(Style::default())
                .graph_type(GraphType::Line)
                .data(&zeropoint),
            if mode % 2 > 0 {
                Dataset::default()
                    .name("v")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Yellow))
                    .graph_type(GraphType::Scatter)
                    .data(&arrayv) }
            else {Dataset::default()},
            if mode % 4 > 1 {
                Dataset::default()
                    .name("a")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Red))
                    .graph_type(GraphType::Scatter)
                    .data(&arraya)
            }
            else {Dataset::default()},
            if mode % 8 > 3 {
                Dataset::default()
                    .name("j")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Green))
                    .graph_type(GraphType::Scatter)
                    .data(&arrayj)
            }
            else {Dataset::default()},
            if mode % 16 > 7 {
                Dataset::default()
                    .name("s")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Blue))
                    .graph_type(GraphType::Scatter)
                    .data(&arrays)
            }
            else {Dataset::default()}
        ];

        let chart = Chart::new(datasets)
            .block(Block::default())
            .x_axis(
                Axis::default()
                    .title(x_labels)
                    .style(Style::default().fg(Color::Gray))
                    .labels(["WHAT!".bold()])
                    .bounds(self.window),
            )
            .y_axis(
                Axis::default()
                    .title("")
                    .style(Style::default().fg(Color::Gray))
                    .labels([inputsens.to_string().into(), mode.to_string().into(), "".bold()])
                    .bounds([-90.0, 90.0]),
            );

        frame.render_widget(&chart, area);
    }
}