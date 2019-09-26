use chrono::prelude::*;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use structopt::StructOpt;

const TOGGL_API: &str = "https://www.toggl.com/api/v8";

#[allow(dead_code)]
fn toggl_api_me() -> String {
    format!("{}/me", TOGGL_API)
}

fn toggl_api_time_entries() -> String {
    format!("{}/time_entries", TOGGL_API)
}

fn read_api_token() -> String {
    fs::read_to_string("api_token")
        .expect("Reading API token from 'api_token' failed")
        .trim()
        .to_string()
}

#[allow(dead_code)]
fn get_user_profile(api_token: String) -> Result<TogglProfile, Error> {
    let client = reqwest::Client::new();
    let new_profile: TogglProfile = client
        .get(&toggl_api_me())
        .basic_auth(api_token, Some("api_token"))
        .send()?
        .json()?;
    Ok(new_profile)
}

fn add_time_entry(api_token: String, time_entry: TimeEntryRequest) {
    let client = reqwest::Client::new();
    let mut response = client
        .post(&toggl_api_time_entries())
        .basic_auth(api_token, Some("api_token"))
        .json(&time_entry)
        .send()
        .unwrap();
    let status = response.status();
    // println!("{:?}", response);
    // println!("{}", response.text().unwrap());
    if status != 200 {
        println!("{}", response.text().expect("Couldn't get response text"));
        println!("{}", status);
        panic!("Aborting, status wasn't 200!");
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let opt = Opt::from_args();
    // println!("{:?}", opt);
    let api_token = read_api_token();
    // println!("{:?}", api_token);
    let time_entry = opt_to_time_entry(opt);
    // println!("{:?}", time_entry);
    add_time_entry(api_token, time_entry);

    // let user_profile = get_user_profile(api_token)?;
    // println!("{:?}", user_profile);
    // let worker_id = user_profile.data.id;
    Ok(())
}

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    // /// Action to perform. Only "add" right now.
    // #[structopt(short = "a", long = "action")]
    // action: String,
    /// Description for task
    #[structopt(short = "d", long = "description")]
    description: String,

    // 1856420
    /// Workspace id
    #[structopt(short = "wid", long = "workspace-id")]
    workspace: u32,

    /// Project id
    #[structopt(short = "pid", long = "project-id")]
    project: u32,

    /// Project id
    #[structopt(long = "start")]
    start: DateTime<Local>,

    /// Project id
    #[structopt(long = "stop")]
    stop: DateTime<Local>,
}

fn opt_to_time_entry(opt: Opt) -> TimeEntryRequest {
    if opt.stop < opt.start {
        panic!("The stop time must be _after_ the start time!");
    }
    let duration = opt.stop - opt.start;
    let duration_seconds = duration.num_seconds() as u32;
    let time_entry = TimeEntry {
        description: opt.description,
        created_with: "Knob".to_string(),
        wid: opt.workspace,
        pid: opt.project,
        start: opt.start,
        duration: duration_seconds,
    };
    TimeEntryRequest {
        time_entry: time_entry,
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TimeEntryRequest {
    time_entry: TimeEntry,
}

#[derive(Serialize, Deserialize, Debug)]
struct TimeEntry {
    description: String,
    created_with: String,
    start: DateTime<Local>,
    duration: u32,
    wid: u32,
    pid: u32,
}

#[derive(Serialize, Debug, Deserialize)]
struct TogglProfile {
    since: i64,
    data: Data,
}

#[derive(Serialize, Debug, Deserialize)]
struct CsvUpload {
    at: String,
    log_id: i64,
}

#[derive(Serialize, Debug, Deserialize)]
struct Data {
    id: i64,
    api_token: String,
    default_wid: i64,
    email: String,
    fullname: String,
    jquery_timeofday_format: String,
    jquery_date_format: String,
    timeofday_format: String,
    date_format: String,
    store_start_and_stop_time: bool,
    beginning_of_week: i64,
    language: String,
    image_url: String,
    sidebar_piechart: bool,
    at: String,
    created_at: String,
    retention: i64,
    record_timeline: bool,
    render_timeline: bool,
    timeline_enabled: bool,
    timeline_experiment: bool,
    new_blog_post: NewBlogPost,
    should_upgrade: bool,
    achievements_enabled: bool,
    timezone: String,
    openid_enabled: bool,
    send_product_emails: bool,
    send_weekly_report: bool,
    send_timer_notifications: bool,
    last_blog_entry: String,
    // Ignore this since we don't know what it looks like
    // invitation: (),
    workspaces: Vec<Workspaces>,
    duration_format: String,
    obm: Obm,
}

#[derive(Serialize, Debug, Deserialize)]
struct NewBlogPost {
    title: String,
    url: String,
    category: String,
    pub_date: String,
}

#[derive(Serialize, Debug, Deserialize)]
struct Obm {
    included: bool,
    nr: i64,
    actions: String,
}

#[derive(Serialize, Debug, Deserialize)]
struct Workspaces {
    id: i64,
    name: String,
    profile: i64,
    premium: bool,
    admin: bool,
    default_hourly_rate: i64,
    default_currency: String,
    only_admins_may_create_projects: bool,
    only_admins_see_billable_rates: bool,
    only_admins_see_team_dashboard: bool,
    projects_billable_by_default: bool,
    rounding: i64,
    rounding_minutes: i64,
    api_token: Option<String>,
    at: String,
    ical_enabled: bool,
    logo_url: Option<String>,
    ical_url: Option<String>,
    csv_upload: Option<CsvUpload>,
}
