use crate::pentane::log_message;
use discord_presence::{Client, models::ActivityType};
use pentane::{PentaneSemVer, PentaneUUID, PluginInformation};

use std::sync::mpsc::{Receiver, Sender, channel};
use std::{time::Duration, thread};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use discord_presence::models::ActivityAssets;
use once_cell::sync::OnceCell;

use sunset_rs::*;

mod pentane;

#[unsafe(no_mangle)]
#[used]
pub static Pentane_PluginInformation: PluginInformation = PluginInformation::new(
    b"Cars 2 Rich Presence",
    b"RiskiVR & memoryleak",
    PentaneUUID::from_str("ad7d9a00c72611f08de90242ac120002"),
    PentaneSemVer::new(0, 1, 0),
    PentaneSemVer::new(1, 0, 0),
);

#[unsafe(no_mangle)]
#[used]
// i don't need any dependencies... i think
pub static Pentane_PluginDependencyCount: usize = 0;

#[derive(Debug)]
pub enum RPCEvent {
    InFrontend(bool),
    CurrentLevel(String),
    // MissionMode(String), unused
}

#[derive(Default)]
pub struct RPCState {
    in_frontend: bool,
    current_level: String,
    mission_mode: String, // unused
}

static EVENT_TX: OnceCell<Sender<RPCEvent>> = OnceCell::new();
static LEVEL_MAP: OnceCell<HashMap<&'static str, &'static str>> = OnceCell::new();

fn map_level_names() -> &'static HashMap<&'static str, &'static str> {
    LEVEL_MAP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("TRACK_A_OilRig", "Pipeline Sprint");
        map.insert("TRACK_B_OilRig", "Oil Rig Run");
        map.insert("TRACK_A_TokyoXtreme", "Ginza Sprint");
        map.insert("TRACK_B_TokyoXtreme", "Vista Run");
        map.insert("TRACK_C_TokyoXtreme", "Imperial Tour");
        map.insert("TRACK_A_Air", "Terminal Sprint");
        map.insert("TRACK_B_Air", "Runway Tour");
        map.insert("TRACK_A_Italy", "Harbor Sprint");
        map.insert("TRACK_B_Italy", "Mountain Run");
        map.insert("TRACK_C_Italy", "Casino Tour");
        map.insert("TRACK_A_London", "Buckingham Sprint");
        map.insert("TRACK_B_London", "Hyde Tour");
        map.insert("TRACK_A_RadiatorSprings", "Radiator Sprint");
        map.insert("TRACK_B_RadiatorSprings", "Canyon Run");
        map.insert("TRACK_C_RadiatorSprings", "Timberline Sprint");
        map.insert("MI_Oil", "Oil Rig Arena");
        map.insert("MI_Air", "Airport Arena");
        map.insert("MI_Italy", "Italy Arena");
        map.insert("MI_London", "London Arena");
        map.insert("MI_Tokyo", "Tokyo Arena");
        map.insert("MI_Radiator", "Radiator Springs Arena");
        map
    })
}

pub fn get_display_name(internal_name: &str) -> String {
    let mut map = map_level_names();

    match map.get(internal_name) {
        Some(display_name) => display_name.to_string(),
        None => format!("Unknown level {}", internal_name),
    }
}

fn init_rpc() -> Client {
    let mut client = Client::new(1380106054146195526);

    client.on_ready(|_ctx| {println!("[RichPresence] Ready!");}).persist();
    client.on_connected(|_ctx|{println!("[RichPresence] Connected!")}).persist();
    client.on_disconnected(|_ctx|{println!("[RichPresence] Disconnected.")}).persist();
    client.on_error(|ctx| eprintln!("[RichPresence] An error has occured while connecting to Discord: {:?}", ctx.event)).persist();

    client.start();

    client
}


fn update_rpc(client: &mut Client, in_frontend: bool, current_level: &String) {
    let result = if in_frontend || current_level.is_empty() {
        client.set_activity(|act| {
            act.state("In Menus")
                .activity_type(ActivityType::Playing)
        })
    } else {
        client.set_activity(|act| {
            let mut assets = ActivityAssets::new();
            act.state("In Race - [Mode]")
                .activity_type(ActivityType::Playing)
                .details(get_display_name(current_level))
                .assets(|_|
                    assets.large_image(current_level.to_lowercase()))
        })
    };

    match result {
        Ok(_) => println!("[RichPresence] Successfully set activity"),
        Err(e) => eprintln!("[RichPresence] Failed to set activity {:?}", e),
    }
}

fn spawn_worker(rx: Receiver<RPCEvent>) {
    println!("[RichPresence] Spawned worker thread");
    let mut state = RPCState::default();

    thread::spawn(move || {
        let mut client = init_rpc();
        loop {
            match rx.recv_timeout(Duration::from_millis(100)){
                Ok(event) => {
                    println!("[RichPresence] Received an event! {:?}", event);
                    match event {
                        RPCEvent::InFrontend(frontend) => {
                            state.in_frontend = frontend;
                        }
                        RPCEvent::CurrentLevel(current_level) => {
                            state.current_level = current_level;
                        }
                    }
                    update_rpc(&mut client, state.in_frontend, &state.current_level);
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    // do nothing! this is normal
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    println!("[RichPresence] Sender disconnected. Exiting update thread");
                    break;
                }
            }
        }
    });
}

#[sunset_rs::hook(offset = 0x004ba0a0)]
pub extern "thiscall" fn carsfrontend_enter_hook(this: *mut ()) {
    if let Some(tx) = EVENT_TX.get() {
        if let Err(error) = tx.send(RPCEvent::InFrontend(true)) {
            println!("[RichPresence] Failed to send event: {:?}", error)
        }
    }
    original!()(this);
}

#[sunset_rs::hook(offset = 0x004bae60)]
pub extern "thiscall" fn carsfrontend_exit_hook(this: *mut ()) {
    if let Some(tx) = EVENT_TX.get() {
        if let Err(error) = tx.send(RPCEvent::InFrontend(false)) {
            println!("[RichPresence] Failed to send event: {:?}", error)
        }
    }
    original!()(this);
}


#[sunset_rs::hook(offset = 0x004c0440)]
pub extern "thiscall" fn carsfrontend_setlevel_hook(this: *mut (), level: *const std::os::raw::c_char) {
    unsafe {
        if let Some(tx) = EVENT_TX.get() {
            let current_level = CStr::from_ptr(level).to_string_lossy().to_string();
            if let Err(error) = tx.send(RPCEvent::CurrentLevel(current_level)) {
                println!("[RichPresence] Failed to send event: {:?}", error)
            }
        }
    }

    original!()(this, level);
}

#[unsafe(no_mangle)]
extern "C" fn Pentane_Main() {
    let (tx, rx) = channel::<RPCEvent>();
    spawn_worker(rx);
    EVENT_TX.set(tx).unwrap();

    sunset_rs::install_hooks!(
		carsfrontend_enter_hook,
		carsfrontend_exit_hook,
        carsfrontend_setlevel_hook
    );
    println!("[RichPresence] Installed hooks!");
}