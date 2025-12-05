use crate::pentane::log_message;
use discord_presence::client;
use pentane::{PentaneSemVer, PentaneUUID, PluginInformation};
use std::sync::Mutex;
use std::sync::mpsc::{Receiver, RecvError, Sender, channel};
use std::{time::Duration, thread};
use discord_presence::{Client, models::ActivityType};
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

static mut EVENT_TX: Option<Mutex<Sender<RPCEvent>>> = None;
static RPC: Mutex<Option<Client>> = Mutex::new(None);

pub fn init_rpc(){
    let mut locked = RPC.lock().unwrap();
    if locked.is_some() {
        return;
    }

    let mut drpc = Client::new(1380106054146195526);

    drpc.on_ready(|_ctx| {println!("[RichPresence] Ready");}).persist();
    drpc.on_connected(|_ctx|{println!("[RichPresence] Connected")}).persist();
    drpc.on_disconnected(|_ctx|{println!("[RichPresence] god fucking dammit god fucking dammit god fu")}).persist();
    drpc.on_error(|ctx| eprintln!("[RichPresence] error: {:?}", ctx.event)).persist();

    drpc.start();
    *locked = Some(drpc);
}

fn update_rpc(client: &mut Client, in_frontend: bool, current_level: String) {
    if in_frontend == true {
        client.set_activity(|act| {
            act.state("In Menus")
                .activity_type(ActivityType::Playing)
        });
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
        println!("[RichPresence] Spawned worker thread!");
        init_rpc();
        let mut locked = RPC.lock().unwrap();

        loop {
            match rx.recv() {
                Ok(event) => {
                    if let Some(client) = locked.as_mut() {
                        match event {
                            RPCEvent::InFrontend(in_frontend) => {
                                update_rpc(client, in_frontend, "".to_string());
                            }
                            RPCEvent::CurrentLevel(current_level) => {
                                update_rpc(client, false, current_level); // passing dummies for now
                            }
                        }
                    }
                },
                Err(e) => println!("Failed to receive event: {:?}", e),
            }
        }
        
    });
}

#[sunset_rs::hook(offset = 0x004c0440)]
pub extern "thiscall" fn carsfrontend_setlevel_hook(this: *mut (), level: *const std::os::raw::c_char) {
    unsafe {
        if let Some(tx) = &EVENT_TX {
            let current_level = std::ffi::CStr::from_ptr(level).to_string_lossy().to_string();
            let _ = tx.lock().unwrap().send(RPCEvent::CurrentLevel(current_level));
        }
    }

    original!()(this, level);
}

#[sunset_rs::hook(offset = 0x00e9dd40)]
pub extern "thiscall" fn frontend_infrontend_hook(this_ptr: *mut ()) -> u8 {
    let raw: u8 = original!()(this_ptr);
    let value = raw != 0;

    unsafe {
        if let Some(tx) = &EVENT_TX {
            let _ = tx.lock().unwrap().send(RPCEvent::InFrontend(value));
        }
    }

    raw
}

#[unsafe(no_mangle)]
extern "C" fn Pentane_Main() {
    let (tx, rx) = channel::<RPCEvent>();

    unsafe {
        EVENT_TX = Some(Mutex::new(tx));
    }

    spawn_worker(rx);

    sunset_rs::install_hooks!(frontend_infrontend_hook, carsfrontend_setlevel_hook);
    println!("[RichPresence] Installed hooks");
}