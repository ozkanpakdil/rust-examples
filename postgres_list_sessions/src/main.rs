// check https://github.com/vectordotdev/vector/blob/master/src/internal_events/postgresql_metrics.rs
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Sparkline},
};
use std::net::IpAddr;
use postgres::{Client, NoTls};
use rust_decimal::Decimal;

// for more information on the query
const SELECT_SESSIONS: &'static str = "SELECT
    pid,
    datname,
    usename,
    application_name,
    client_addr,
    pg_catalog.to_char(backend_start, 'YYYY-MM-DD HH24:MI:SS TZ') AS backend_start,
    state,
    wait_event_type || ': ' || wait_event AS wait_event,
    array_to_string(pg_catalog.pg_blocking_pids(pid), ', ') AS blocking_pids,
    query,
    pg_catalog.to_char(state_change, 'YYYY-MM-DD HH24:MI:SS TZ') AS state_change,
    pg_catalog.to_char(query_start, 'YYYY-MM-DD HH24:MI:SS TZ') AS query_start,
    pg_catalog.to_char(xact_start, 'YYYY-MM-DD HH24:MI:SS TZ') AS xact_start,
    backend_type,
    CASE WHEN state = 'active' THEN ROUND((extract(epoch from now() - query_start) / 60)::numeric, 2) ELSE 0 END AS active_since
FROM
    pg_catalog.pg_stat_activity
ORDER BY pid;";

#[derive(Clone)]
struct RandomSignal {
    distribution: Uniform<u64>,
    rng: ThreadRng,
}

impl RandomSignal {
    fn new(lower: u64, upper: u64) -> Self {
        Self {
            distribution: Uniform::new(lower, upper),
            rng: rand::thread_rng(),
        }
    }
}
impl Iterator for RandomSignal {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(self.distribution.sample(&mut self.rng))
    }
}

struct App {
    signal: RandomSignal,
    data1: Vec<u64>,
    data2: Vec<u64>,
    data3: Vec<u64>,
}
impl App {
    fn new() -> Self {
        let mut signal = RandomSignal::new(0, 100);
        let data1 = signal.by_ref().take(200).collect::<Vec<u64>>();
        let data2 = signal.by_ref().take(200).collect::<Vec<u64>>();
        let data3 = signal.by_ref().take(200).collect::<Vec<u64>>();
        Self {
            signal,
            data1,
            data2,
            data3,
        }
    }

    fn on_tick(&mut self) {
        let value = self.signal.next().unwrap();
        self.data1.pop();
        self.data1.insert(0, value);
        let value = self.signal.next().unwrap();
        self.data2.pop();
        self.data2.insert(0, value);
        let value = self.signal.next().unwrap();
        self.data3.pop();
        self.data3.insert(0, value);
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(0),
    ])
        .split(f.size());
    let sparkline = Sparkline::default()
        .block(
            Block::new()
                .borders(Borders::LEFT | Borders::RIGHT)
                .title("Data1"),
        )
        .data(&app.data1)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(sparkline, chunks[0]);
    let sparkline = Sparkline::default()
        .block(
            Block::new()
                .borders(Borders::LEFT | Borders::RIGHT)
                .title("Data2"),
        )
        .data(&app.data2)
        .style(Style::default().bg(Color::Green));
    f.render_widget(sparkline, chunks[1]);
    // Multiline
    let sparkline = Sparkline::default()
        .block(
            Block::new()
                .borders(Borders::LEFT | Borders::RIGHT)
                .title("Data3"),
        )
        .data(&app.data3)
        .style(Style::default().fg(Color::Red));
    f.render_widget(sparkline, chunks[2]);
}
fn print_all_sessions() {
    let sessions = bring_sessions().unwrap();
    for row in sessions {
        let pid: i32 = row.get(0);
        let datname: String = row.get(1);
        let usename: String = row.get(2);
        let application_name: String = row.get(3);
        let client_addr: Option<IpAddr> = row.get(4);
        let backend_start: String = row.get(5);
        let state: String = row.get(6);
        let wait_event: Option<String> = row.get(7);
        let blocking_pids: Option<Vec<i32>> = row.get(8);
        let query: Option<String> = row.get(9);
        let state_change: Option<String> = row.get(10);
        let query_start: Option<String> = row.get(11);
        let xact_start: Option<String> = row.get(12);
        let backend_type: String = row.get(13);
        let active_since: Decimal = row.get(14);

        println!(
            "{} | {} | {} | {} | {:?} | {} | {} | {:?} | {:?} | {:?} | {:?} | {:?} | {:?} | {} | {}",
            pid,
            datname,
            usename,
            application_name,
            client_addr,
            backend_start,
            state,
            wait_event,
            blocking_pids,
            query,
            state_change,
            query_start,
            xact_start,
            backend_type,
            active_since
        );
    }
}
fn bring_sessions() -> Result<Vec<postgres::Row>, postgres::Error> {
    let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

    let headers = vec![
        "pid",
        "datname",
        "usename",
        "application_name",
        "client_addr",
        "backend_start",
        "state",
        "wait_event",
        "blocking_pids",
        "query",
        "state_change",
        "query_start",
        "xact_start",
        "backend_type",
        "active_since",
    ];

    // Print the headers
    println!("{}", headers.join(" | "));

    match client.query(SELECT_SESSIONS, &[]) {
        Ok(rows) => {
            return Ok(rows);
        }
        Err(e) => {
            eprintln!("An error occurred: {}", e);
            return Err(e);
        }
    }
}
