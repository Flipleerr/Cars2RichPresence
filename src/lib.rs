use crate::pentane::log_message;
use discord_presence::{Client, models::ActivityType};
use pentane::{PentaneSemVer, PentaneUUID, PluginInformation};

use std::sync::mpsc::{Receiver, Sender, channel};
use std::{time::Duration, thread};
use std::ffi::CStr;
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
    CurrentLevel(String),
    InFrontend(bool),
}
static EVENT_TX: OnceCell<Sender<RPCEvent>> = OnceCell::new();

fn init_rpc() -> Client {
    let mut client = Client::new(1380106054146195526);

    client.on_ready(|_ctx| {println!("[RichPresence] Ready!");}).persist();
    client.on_connected(|_ctx|{println!("[RichPresence] Connected!")}).persist();
    client.on_disconnected(|_ctx|{println!("[RichPresence] god fucking dammit god fucking dammit god fu")}).persist();
    client.on_error(|ctx| eprintln!("[RichPresence] error: {:?}", ctx.event)).persist();

    client.start();

    client
}

fn update_rpc(client: &mut Client, in_frontend: bool, current_level: &String) {
    if in_frontend || current_level.is_empty() {
        match client.set_activity(|act| {
        act.state("In Menus")
            .activity_type(ActivityType::Playing)
        }) {
            Ok(act) => println!("[RichPresence] Successfully set activity"),
            Err(e) => println!("[RichPresence] Failed to set activity {:?}", e)
        };
    } else {
        client.set_activity(|act| {
        act.state("In Race - [Mode]")
            .activity_type(ActivityType::Playing)
            .details(current_level)
        });
    }
}

fn spawn_worker(rx: Receiver<RPCEvent>) {
    thread::spawn(move || {
        println!("[RichPresence] Spawned worker thread");
        let mut client = init_rpc();
        let mut last_frontend_state = true;
        let mut last_level_state = String::new();

        loop {
            println!("[RichPresence] Waiting...");
            match rx.recv() {
                Ok(event) => {
                    println!("[RichPresence] Received an event! {:?}", event);
                    match event {
                        RPCEvent::InFrontend(frontend) => {
                            last_frontend_state = frontend;
                        }
                        RPCEvent::CurrentLevel(current_level) => {
                            last_level_state = current_level;
                        }
                    }

                    println!("[RichPresence] Current state: is in frontend - {}, current level - {}", last_frontend_state, last_level_state);
                    update_rpc(&mut client, last_frontend_state, &last_level_state);

                    thread::sleep(Duration::from_millis(1500));
                }

                Err(e) => {
                    println!("Failed to receive event! {:?} Exiting worker thread", e);
                    break;
                }
            }
        }
        println!("[RichPresence] Worker thread has closed");
    });
}

#[sunset_rs::hook(offset = 0x004c0440)]
pub extern "thiscall" fn carsfrontend_setlevel_hook(this: *mut (), level: *const std::os::raw::c_char) {
    unsafe {
        if let Some(tx) = EVENT_TX.get() {
            let current_level = CStr::from_ptr(level).to_string_lossy().to_string();
            let result = tx.send(RPCEvent::CurrentLevel(current_level));

            match result {
                Ok(()) => println!("[RichPresence] Detected level {}! Sending event", CStr::from_ptr(level).to_string_lossy().to_string()),
                Err(e) => println!("[RichPresence] Failed to send event: {:?}", e)
            }
        }
    }

    original!()(this, level);
}

#[sunset_rs::hook(offset = 0x00e9dd40)]
pub extern "thiscall" fn frontend_infrontend_hook(this_ptr: *mut ()) -> u8 {
    let raw: u8 = original!()(this_ptr);
    unsafe {
        if let Some(tx) = EVENT_TX.get() {
            let value = raw != 0;
            let result = tx.send(RPCEvent::InFrontend(value));

            match result {
                Ok(()) => println!("[RichPresence] Detected frontend state {}! Sending event", value),
                Err(e) => println!("[RichPresence] Failed to send event: {:?}", e)
            }
        }
    }
    raw
}

#[unsafe(no_mangle)]
extern "C" fn Pentane_Main() {
    let (tx, rx) = channel::<RPCEvent>();
    spawn_worker(rx);
    EVENT_TX.set(tx).unwrap();

    sunset_rs::install_hooks!(
        frontend_infrontend_hook,
        carsfrontend_setlevel_hook
    );
    println!("[RichPresence] Installed hooks!");
}
