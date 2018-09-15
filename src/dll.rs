use ffi::*;
use libloading::Library;
use std::error::Error;
use std::io;
use std::os::raw::{c_int, c_uint, c_void};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;

// These type signatures came from bindgen.
type UpdatePlayerInput = extern "C" fn(PlayerInput, c_int) -> RLBotCoreStatus;
type UpdatePlayerInputFlatbuffer = extern "C" fn(*mut c_void, c_int) -> RLBotCoreStatus;
type UpdateLiveDataPacket = extern "C" fn(*mut LiveDataPacket) -> RLBotCoreStatus;
type UpdateLiveDataPacketFlatbuffer = extern "C" fn() -> ByteBuffer;
type StartMatch = extern "C" fn(MatchSettings, CallbackFunction, *mut c_uint) -> RLBotCoreStatus;
type IsInitialized = extern "C" fn() -> bool;
type UpdateFieldInfo = extern "C" fn(*mut FieldInfo) -> RLBotCoreStatus;
type UpdateFieldInfoFlatbuffer = extern "C" fn() -> ByteBuffer;
type SendChat =
    extern "C" fn(QuickChatPreset, c_int, bool, CallbackFunction, *mut c_uint) -> RLBotCoreStatus;
type SendQuickChat = extern "C" fn(*mut c_void, c_int) -> RLBotCoreStatus;
type SetGameState = extern "C" fn(*mut c_void, c_int) -> RLBotCoreStatus;
type RenderGroup = extern "C" fn(*mut c_void, c_int) -> RLBotCoreStatus;

/// Tracks whether RLBot_Core_Interface has been loaded into this process.
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct RLBotCoreInterface {
    pub update_player_input: UpdatePlayerInput,
    pub update_player_input_flatbuffer: UpdatePlayerInputFlatbuffer,
    pub update_live_data_packet: UpdateLiveDataPacket,
    pub update_live_data_packet_flatbuffer: UpdateLiveDataPacketFlatbuffer,
    pub update_field_info: UpdateFieldInfo,
    pub update_field_info_flatbuffer: UpdateFieldInfoFlatbuffer,
    pub set_game_state: SetGameState,
    pub render_group: RenderGroup,
    pub send_chat: SendChat,
    /// Flatbuffer version of send_chat
    pub send_quick_chat: SendQuickChat,
    pub start_match: StartMatch,
    pub is_initialized: IsInitialized,
}

impl RLBotCoreInterface {
    pub fn load() -> io::Result<RLBotCoreInterface> {
        if INITIALIZED.swap(true, Ordering::SeqCst) {
            panic!("RLBot can only be initialized once");
        }

        let library = Library::new("RLBot_Core_Interface.dll")?;

        // This DLL does not seem to clean itself up all the way when unloaded, so to
        // avoid segfaults/etc we need to make sure it stays loaded until the process
        // exits.
        let library = Box::leak(Box::new(library));

        unsafe {
            Ok(RLBotCoreInterface {
                update_player_input: *library.get(b"UpdatePlayerInput")?,
                update_player_input_flatbuffer: *library.get(b"UpdatePlayerInputFlatbuffer")?,
                update_live_data_packet: *library.get(b"UpdateLiveDataPacket")?,
                update_live_data_packet_flatbuffer: *library
                    .get(b"UpdateLiveDataPacketFlatbuffer")?,
                update_field_info: *library.get(b"UpdateFieldInfo")?,
                update_field_info_flatbuffer: *library.get(b"UpdateFieldInfoFlatbuffer")?,
                set_game_state: *library.get(b"SetGameState")?,
                render_group: *library.get(b"RenderGroup")?,
                send_chat: *library.get(b"SendChat")?,
                send_quick_chat: *library.get(b"SendQuickChat")?,
                start_match: *library.get(b"StartMatch")?,
                is_initialized: *library.get(b"IsInitialized")?,
            })
        }
    }

    pub fn wait_for_initialized(&self) -> Result<(), Box<Error>> {
        for _ in 0..100 {
            if (self.is_initialized)() {
                return Ok(());
            }
            sleep(Duration::from_millis(10));
        }

        Err(From::from("RLBot did not become initialized"))
    }
}
