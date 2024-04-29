use json::{object, array, JsonValue};
use crate::router::global;
use actix_web::{HttpResponse, HttpRequest};
use rusqlite::{Connection, params, ToSql};
use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;

lazy_static! {
    static ref ENGINE: Mutex<Option<Connection>> = Mutex::new(None);
}
fn init(engine: &mut MutexGuard<'_, Option<Connection>>) {
    let conn = Connection::open("live_statistics.db").unwrap();
    conn.execute("PRAGMA foreign_keys = ON;", ()).unwrap();

    engine.replace(conn);
}
fn lock_and_exec(command: &str, args: &[&dyn ToSql]) {
    loop {
        match ENGINE.lock() {
            Ok(mut result) => {
                if result.is_none() {
                    init(&mut result);
                }
                let conn = result.as_ref().unwrap();
                conn.execute(command, args).unwrap();
                return;
            }
            Err(_) => {
                std::thread::sleep(std::time::Duration::from_millis(15));
            }
        }
    }
}
fn lock_and_select_all(command: &str, args: &[&dyn ToSql]) -> Result<JsonValue, rusqlite::Error> {
    loop {
        match ENGINE.lock() {
            Ok(mut result) => {
                if result.is_none() {
                    init(&mut result);
                }
                let conn = result.as_ref().unwrap();
                let mut stmt = conn.prepare(command)?;
                let map = stmt.query_map(args, |row| {
                    match row.get::<usize, i64>(0) {
                        Ok(val) => Ok(val.to_string()),
                        Err(_) => row.get(0)
                    }
                })?;
                let mut rv = array![];
                for val in map {
                    let res = val?;
                    match res.clone().parse::<i64>() {
                        Ok(v) => rv.push(v).unwrap(),
                        Err(_) => rv.push(res).unwrap()
                    };
                }
                return Ok(rv);
            }
            Err(_) => {
                std::thread::sleep(std::time::Duration::from_millis(15));
            }
        }
    }
}

struct Live {
    live_id: i32,
    normal_failed: i64,
    normal_pass: i64,
    hard_failed: i64,
    hard_pass: i64,
    expert_failed: i64,
    expert_pass: i64,
    master_failed: i64,
    master_pass: i64,
}

fn create_store() {
    lock_and_exec("CREATE TABLE IF NOT EXISTS lives (
        live_id         INT NOT NULL PRIMARY KEY,
        normal_failed   BIGINT NOT NULL,
        normal_pass     BIGINT NOT NULL,
        hard_failed     BIGINT NOT NULL,
        hard_pass       BIGINT NOT NULL,
        expert_failed   BIGINT NOT NULL,
        expert_pass     BIGINT NOT NULL,
        master_failed   BIGINT NOT NULL,
        master_pass     BIGINT NOT NULL
    )", params!());
}

fn get_live_data(id: i64) -> Result<Live, rusqlite::Error> {
    loop {
        match ENGINE.lock() {
            Ok(mut result) => {
                if result.is_none() {
                    init(&mut result);
                }
                let conn = result.as_ref().unwrap();
                
                let mut stmt = conn.prepare("SELECT * FROM lives WHERE live_id=?1")?;
                return Ok(stmt.query_row(params!(id), |row| {
                    Ok(Live {
                        live_id: row.get(0)?,
                        normal_failed: row.get(1)?,
                        normal_pass: row.get(2)?,
                        hard_failed: row.get(3)?,
                        hard_pass: row.get(4)?,
                        expert_failed: row.get(5)?,
                        expert_pass: row.get(6)?,
                        master_failed: row.get(7)?,
                        master_pass: row.get(8)?,
                    })
                })?);
            }
            Err(_) => {
                std::thread::sleep(std::time::Duration::from_millis(15));
            }
        }
    }
}

pub fn live_completed(id: i64, level: i32, failed: bool) {
    //let live = get_live_id(id);
    match get_live_data(id) {
        Ok(info) => {
            let value = format!("{}_{}", 
                if 1 == level { "normal" } else if 2 == level { "hard" } else if 3 == level { "expert" } else { "master" },
                if failed { "failed" } else { "pass" }
            );
            let new_info = if 1 == level && failed { info.normal_failed }
                           else if 1 == level && !failed { info.normal_pass }
                           else if 2 == level && failed { info.hard_failed }
                           else if 2 == level && !failed { info.hard_pass }
                           else if 3 == level && failed { info.expert_failed }
                           else if 3 == level && !failed { info.expert_pass }
                           else if 4 == level && failed { info.master_failed }
                           else if 4 == level && !failed { info.master_pass } else { return; };
            
            lock_and_exec(&format!("UPDATE lives SET {}=?1 WHERE live_id=?2", value), params!(new_info + 1, info.live_id));
        },
        Err(_) => {
            lock_and_exec("INSERT INTO lives (live_id, normal_failed, normal_pass, hard_failed, hard_pass, expert_failed, expert_pass, master_failed, master_pass) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)", params!(
                id,
                if 1 == level && failed { 1 } else { 0 },
                if 1 == level && !failed { 1 } else { 0 },
                if 2 == level && failed { 1 } else { 0 },
                if 2 == level && !failed { 1 } else { 0 },
                if 3 == level && failed { 1 } else { 0 },
                if 3 == level && !failed { 1 } else { 0 },
                if 4 == level && failed { 1 } else { 0 },
                if 4 == level && !failed { 1 } else { 0 }
            ));
        },
    };
}

lazy_static! {
    static ref CACHED_DATA: Mutex<Option<JsonValue>> = Mutex::new(None);
    static ref LIVE_LIST: JsonValue = {
        let mut info = object!{};
        let mut info2 = object!{};
        let items = json::parse(include_str!("json/live.json")).unwrap();
        for (_i, data) in items.members().enumerate() {
            info[data["id"].to_string()] = data["masterMusicId"].clone();
        }
        for (_i, data) in items.members().enumerate() {
            info2[data["masterMusicId"].to_string()] = data["id"].clone();
        }
        object!{
            masterMusicId: info,
            id: info2
        }
    };
}
fn get_live_id(id: i64) -> i64 {
    LIVE_LIST["masterMusicId"][id.to_string()].as_i64().unwrap()
}

fn get_pass_percent(failed: i64, pass: i64) -> String {
    let total = (failed + pass) as f64;
    if failed + pass == 0 {
        return String::from("-/-%");
    }
    let pass = pass as f64;
    format!("{:.2}%", pass / total * 100.0)
}

fn get_json() -> JsonValue {
    let lives = lock_and_select_all("SELECT live_id FROM lives", params!()).unwrap();
    let mut rates = array![];
    let mut ids = array![];
    for (_i, id) in lives.members().enumerate() {
        let info = get_live_data(id.as_i64().unwrap());
        if !info.is_ok() {
            continue;
        }
        let info = info.unwrap();
        let to_push = object!{
            master_live_id: info.live_id,
            normal: get_pass_percent(info.normal_failed, info.normal_pass),
            hard: get_pass_percent(info.hard_failed, info.hard_pass),
            expert: get_pass_percent(info.expert_failed, info.expert_pass),
            master: get_pass_percent(info.master_failed, info.master_pass)
        };
        ids.push(get_live_id(info.live_id.into())).unwrap();
        rates.push(to_push).unwrap();
    }
    object!{
        "cache": {
            "all_user_clear_rate": rates,
            "master_music_ids": ids,
            "event_live_list": []
        },
        "last_updated": global::timestamp()
    }
}

fn get_clearrate_json() -> JsonValue {
    create_store();
    loop {
        match CACHED_DATA.lock() {
            Ok(mut result) => {
                if result.is_none() {
                    result.replace(get_json());
                }
                let cache = result.as_ref().unwrap();
                if cache["last_updated"].as_u64().unwrap() + (60 * 60) > global::timestamp() {
                    return cache["cache"].clone();
                }
                let new = get_json();
                result.replace(new.clone());
                return new["cache"].clone();
            }
            Err(_) => {
                std::thread::sleep(std::time::Duration::from_millis(15));
            }
        }
    }
}

pub fn clearrate(_req: HttpRequest) -> HttpResponse {
    let resp = object!{
        "code": 0,
        "server_time": global::timestamp(),
        "data": get_clearrate_json()
    };
    global::send(resp)
}