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
    InFrontend(bool),
    CurrentLevel(String),
}

#[derive(Default)]
pub struct RPCState {
    in_frontend: bool,
    current_level: String,
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
    let result = if in_frontend || current_level.is_empty() {
        client.set_activity(|act| {
        act.state("In Menus")
            .activity_type(ActivityType::Playing)
        })
    } else {
        client.set_activity(|act| {
        act.state("In Race - [Mode]")
            .activity_type(ActivityType::Playing)
            .details(current_level)
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
            println!("[RichPresence] Waiting...");
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
                Ok(()) => (),
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
