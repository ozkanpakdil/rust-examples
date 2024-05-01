use std::net::IpAddr;
use postgres::{Client, NoTls};
use rust_decimal::Decimal;

// check https://github.com/vectordotdev/vector/blob/master/src/internal_events/postgresql_metrics.rs
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

fn main() {
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
            for row in rows {
                let pid: Option<i32> = row.get(0);
                let datname: Option<String> = row.get(1);
                let usename: Option<String> = row.get(2);
                let application_name: Option<String> = row.get(3);
                let client_addr: Option<IpAddr> = row.get(4);
                let backend_start: Option<String> = row.get(5);
                let state: Option<String> = row.get(6);
                let wait_event: Option<String> = row.get(7);
                let blocking_pids: Option<String> = row.get(8);
                let query: Option<String> = row.get(9);
                let state_change: Option<String> = row.get(10);
                let query_start: Option<String> = row.get(11);
                let xact_start: Option<String> = row.get(12);
                let backend_type: Option<String> = row.get(13);
                let active_since: Option<Decimal> = row.get(14);

                println!(
                    "{} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {}",
                    pid.unwrap_or_default(),
                    datname.unwrap_or_default(),
                    usename.unwrap_or_default(),
                    application_name.unwrap_or_default(),
                    client_addr.unwrap_or_else(|| IpAddr::V4("127.0.0.1".parse().unwrap())),
                    backend_start.unwrap_or_default(),
                    state.unwrap_or_default(),
                    wait_event.unwrap_or_default(),
                    blocking_pids.unwrap_or_default(),
                    query.unwrap_or_default(),
                    state_change.unwrap_or_default(),
                    query_start.unwrap_or_default(),
                    xact_start.unwrap_or_default(),
                    backend_type.unwrap_or_default(),
                    active_since.unwrap_or_default()
                );
            }
        }
        Err(e) => {
            eprintln!("An error occurred: {}", e);
        }
    }
}
