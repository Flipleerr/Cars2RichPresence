use std::collections::HashMap;
use once_cell::sync::OnceCell;

static LEVEL_MAP: OnceCell<HashMap<&'static str, &'static str>> = OnceCell::new();
static MODE_MAP: OnceCell<HashMap<&'static str, &'static str>> = OnceCell::new();
static MODE_ICON_MAP: OnceCell<HashMap<&'static str, &'static str>> = OnceCell::new();

fn map_level_names() -> &'static HashMap<&'static str, &'static str> {
    LEVEL_MAP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("TRACK_A_OilRig", "Pipeline Sprint");
        map.insert("TRACK_B_OilRig", "Oil Rig Run");
        map.insert("TRACK_A_TokyoXtreme", "Ginza Sprint");
        map.insert("TRACK_B_TokyoXtreme", "Vista Run");
        map.insert("TRACK_C_TokyoXtreme", "Imperial Tour");
        map.insert("TRACK_A_Air", "Terminal Sprint");
        map.insert("TRACK_C_Air", "Runway Tour");
        map.insert("TRACK_A_Italy", "Harbor Sprint");
        map.insert("TRACK_B_Italy", "Mountain Run");
        map.insert("TRACK_C_Italy", "Casino Tour");
        map.insert("TRACK_A_London", "Buckingham Sprint");
        map.insert("TRACK_B_London", "Hyde Tour");
        map.insert("TRACK_A_RadiatorSprings", "Radiator Sprint");
        map.insert("TRACK_B_RadiatorSprings", "Canyon Run");
        map.insert("TRACK_C_RadiatorSprings", "Timberline Sprint");
        map.insert("Location_MI_Oil", "Oil Rig Arena");
        map.insert("Location_MI_AIR", "Airport Arena");
        map.insert("Location_MI_Italy", "Italy Arena");
        map.insert("Location_MI_London", "London Arena");
        map.insert("Location_MI_Tokyo", "Tokyo Arena");
        map.insert("Location_MI_Radiator", "Radiator Springs Arena");
        map.insert("", "");
        map
    })
}

fn map_mode_names() -> &'static HashMap<&'static str, &'static str> {
    MODE_MAP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("RACE", "Race");
        map.insert("BATTLE_RACE", "Battle Race");
        map.insert("TAKEDOWN", "Attack");
        map.insert("COLLECT", "Survival");
        map.insert("HUNTER", "Hunter");
        map.insert("ARENA", "Arena");
        map.insert("BOMB", "Disruptor");
        map.insert("", "");
        map
    })
}

fn map_mode_icons() -> &'static HashMap<&'static str, &'static str> {
    MODE_ICON_MAP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("RACE", "icon_race");
        map.insert("BATTLE_RACE", "icon_battlerace");
        map.insert("TAKEDOWN", "icon_attack");
        map.insert("COLLECT", "icon_survival");
        map.insert("HUNTER", "icon_hunter");
        map.insert("ARENA", "icon_arena");
        map.insert("BOMB", "icon_disruptor");
        map.insert("", "");
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

pub fn get_mode_name(internal_name: &str) -> String {
    let mut map = map_mode_names();

    match map.get(internal_name) {
        Some(display_name) => display_name.to_string(),
        None => format!("Unknown mode {}", internal_name),
    }
}

pub fn get_mode_icon(mode_name: &str) -> String {
    let mut map = map_mode_icons();

    match map.get(mode_name) {
        Some(icon_name) => icon_name.to_string(),
        None => format!("Unknown mode {}", mode_name),
    }
}
