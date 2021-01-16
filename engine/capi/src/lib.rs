#![allow(clippy::missing_safety_doc)]

pub use kime_engine_core::{
    InputResult,
    InputEngine,
    Config,
};

/// Create new engine
#[no_mangle]
pub extern "C" fn kime_engine_new() -> *mut InputEngine {
    Box::into_raw(Box::new(InputEngine::new()))
}

/// Delete engine
#[no_mangle]
pub unsafe extern "C" fn kime_engine_delete(engine: *mut InputEngine) {
    drop(Box::from_raw(engine));
}

/// Get preedit_char of engine
///
/// ## Return
///
/// valid ucs4 char NULL to represent empty
#[no_mangle]
pub unsafe extern "C" fn kime_engine_preedit_char(engine: *const InputEngine) -> u32 {
    let engine = engine.as_ref().unwrap();

    engine.preedit_char() as u32
}

/// Reset preedit state then returm commit char
///
/// ## Return
///
/// valid ucs4 char NULL to represent empty
#[no_mangle]
pub unsafe extern "C" fn kime_engine_reset(engine: *mut InputEngine) -> u32 {
    let engine = engine.as_mut().unwrap();
    engine.reset() as u32
}

/// Press key when modifier state
///
/// ## Return
///
/// input result
#[no_mangle]
pub unsafe extern "C" fn kime_engine_press_key(
    engine: *mut InputEngine,
    config: *const Config,
    hardware_code: u16,
    state: u32,
) -> InputResult {
    let engine = engine.as_mut().unwrap();
    let config = config.as_ref().unwrap();

    engine.press_key_code(hardware_code, state, config)
}

/// Load config from local file
#[no_mangle]
pub extern "C" fn kime_config_load() -> *mut Config {
    Box::into_raw(Box::new(Config::load_from_config_dir().unwrap_or_default()))
}

/// Delete config
#[no_mangle]
pub unsafe extern "C" fn kime_config_delete(config: *mut Config) {
    drop(Box::from_raw(config));
}

/// Get gtk_commit_english config
///
/// ## Return
///
/// 1 = true, 0 = false
#[no_mangle]
pub unsafe extern "C" fn kime_config_gtk_commit_english(config: *const Config) -> u32 {
    config.as_ref().unwrap().gtk_commit_english.into()
}

/// Get xim_preedit_font config
/// name only valid while config is live
///
/// ## Return
///
/// utf-8 string when len
#[no_mangle]
pub unsafe extern "C" fn kime_config_xim_preedit_font(
    config: *const Config,
    name: *mut *const u8,
    len: *mut usize,
) {
    let font = config.as_ref().unwrap().xim_preedit_font.as_str();
    name.write(font.as_ptr());
    len.write(font.len());
}
