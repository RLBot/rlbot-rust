use crate::{ffi::*, utils::maybe_join};
use libloading::Library;
use std::{
    io,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

// These type signatures came from bindgen.
type UpdateFieldInfoFlatbuffer = extern "C" fn() -> ByteBuffer;
type UpdateFieldInfo = extern "C" fn(pFieldInfo: *mut FieldInfo) -> RLBotCoreStatus;
type UpdateLiveDataPacketFlatbuffer = extern "C" fn() -> ByteBuffer;
type UpdateLiveDataPacket = extern "C" fn(pLiveData: *mut LiveDataPacket) -> RLBotCoreStatus;
type UpdateRigidBodyTickFlatbuffer = extern "C" fn() -> ByteBuffer;
type UpdateRigidBodyTick = extern "C" fn(rigidBodyTick: *mut RigidBodyTick) -> RLBotCoreStatus;
type Free = extern "C" fn(ptr: *mut ::std::os::raw::c_void);
type SetGameState = extern "C" fn(
    gameStateData: *mut ::std::os::raw::c_void,
    size: ::std::os::raw::c_int,
) -> RLBotCoreStatus;
type StartMatch = extern "C" fn(
    matchSettings: MatchSettings,
    callback: CallbackFunction,
    pID: *mut ::std::os::raw::c_uint,
) -> RLBotCoreStatus;
type StartMatchFlatbuffer = extern "C" fn(
    startMatchSettings: *mut ::std::os::raw::c_void,
    size: ::std::os::raw::c_int,
) -> RLBotCoreStatus;
type SendQuickChat = extern "C" fn(
    quickChatMessage: *mut ::std::os::raw::c_void,
    protoSize: ::std::os::raw::c_int,
) -> RLBotCoreStatus;
type SendChat = extern "C" fn(
    quickChatPreset: QuickChatPreset,
    playerIndex: ::std::os::raw::c_int,
    bTeam: bool,
    callback: CallbackFunction,
    pID: *mut ::std::os::raw::c_uint,
) -> RLBotCoreStatus;
type UpdatePlayerInput =
    extern "C" fn(playerInput: PlayerInput, playerIndex: ::std::os::raw::c_int) -> RLBotCoreStatus;
type UpdatePlayerInputFlatbuffer = extern "C" fn(
    playerInput: *mut ::std::os::raw::c_void,
    size: ::std::os::raw::c_int,
) -> RLBotCoreStatus;
type RenderGroup = extern "C" fn(
    renderGroup: *mut ::std::os::raw::c_void,
    protoSize: ::std::os::raw::c_int,
) -> RLBotCoreStatus;
type IsInitialized = extern "C" fn() -> bool;
type GetBallPrediction = extern "C" fn() -> ByteBuffer;
type GetBallPredictionStruct =
    extern "C" fn(pBallPrediction: *mut BallPredictionPacket) -> RLBotCoreStatus;

/// Tracks whether RLBot_Core_Interface has been loaded into this process.
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct RLBotCoreInterface {
    pub update_field_info_flatbuffer: UpdateFieldInfoFlatbuffer,
    pub update_field_info: UpdateFieldInfo,
    pub update_live_data_packet_flatbuffer: UpdateLiveDataPacketFlatbuffer,
    pub update_live_data_packet: UpdateLiveDataPacket,
    pub update_rigid_body_tick_flatbuffer: UpdateRigidBodyTickFlatbuffer,
    pub update_rigid_body_tick: UpdateRigidBodyTick,
    pub free: Free,
    pub set_game_state: SetGameState,
    pub start_match: StartMatch,
    pub start_match_flatbuffer: StartMatchFlatbuffer,
    /// FlatBuffer version of send_chat
    pub send_quick_chat: SendQuickChat,
    pub send_chat: SendChat,
    pub update_player_input: UpdatePlayerInput,
    pub update_player_input_flatbuffer: UpdatePlayerInputFlatbuffer,
    pub render_group: RenderGroup,
    pub is_initialized: IsInitialized,
    pub get_ball_prediction: GetBallPrediction,
    pub get_ball_prediction_struct: GetBallPredictionStruct,
}

impl RLBotCoreInterface {
    pub fn load(rlbot_dll_directory: Option<&Path>) -> io::Result<RLBotCoreInterface> {
        if INITIALIZED.swap(true, Ordering::SeqCst) {
            panic!("RLBot can only be initialized once");
        }

        let path = maybe_join(rlbot_dll_directory, "RLBot_Core_Interface.dll");
        let library = Library::new(&path)
            .or_else(|_| Library::new(path.with_file_name("libRLBotInterface.so")))?;

        // This DLL does not seem to clean itself up all the way when unloaded, so to
        // avoid segfaults/etc we need to make sure it stays loaded until the process
        // exits.
        let library = Box::leak(Box::new(library));

        unsafe {
            Ok(RLBotCoreInterface {
                update_field_info_flatbuffer: *library.get(b"UpdateFieldInfoFlatbuffer")?,
                update_field_info: *library.get(b"UpdateFieldInfo")?,
                update_live_data_packet_flatbuffer: *library
                    .get(b"UpdateLiveDataPacketFlatbuffer")?,
                update_live_data_packet: *library.get(b"UpdateLiveDataPacket")?,
                update_rigid_body_tick_flatbuffer: *library
                    .get(b"UpdateRigidBodyTickFlatbuffer")?,
                update_rigid_body_tick: *library.get(b"UpdateRigidBodyTick")?,
                free: *library.get(b"Free")?,
                set_game_state: *library.get(b"SetGameState")?,
                start_match: *library.get(b"StartMatch")?,
                start_match_flatbuffer: *library.get(b"StartMatchFlatbuffer")?,
                send_quick_chat: *library.get(b"SendQuickChat")?,
                send_chat: *library.get(b"SendChat")?,
                update_player_input: *library.get(b"UpdatePlayerInput")?,
                update_player_input_flatbuffer: *library.get(b"UpdatePlayerInputFlatbuffer")?,
                render_group: *library.get(b"RenderGroup")?,
                is_initialized: *library.get(b"IsInitialized")?,
                get_ball_prediction: *library.get(b"GetBallPrediction")?,
                get_ball_prediction_struct: *library.get(b"GetBallPredictionStruct")?,
            })
        }
    }
}
