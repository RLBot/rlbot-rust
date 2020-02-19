use crate::{
    game::{GameTickPacket, ControllerState},
    init_with_options, InitOptions
};
use std::{env, error::Error, path::PathBuf, cmp::Ordering};

// Most of this is basically a copy of what is in framework.rs,
// just adapted for hivemind.

// Use this like you would use the Bot trait
pub trait Hivemind {
    fn set_drone_indices(&mut self, indices: Vec<usize>);
    fn tick(&mut self, packet: &GameTickPacket) -> Vec<ControllerState>;
}

pub fn run_hive<H: Hivemind>(mut hive: H) -> Result<(), Box<dyn Error>> {
    let args = parse_framework_args()
    .map_err(|_| Box::<dyn Error>::from("could not parse framework arguments"))?
    .ok_or_else(|| Box::<dyn Error>::from("not launched by framework"))?;

    let drone_indices = args.drone_indices.to_vec();
    let rlbot = init_with_options(args.into())?;
    
    // Create a Vec<usize> copy of the original Vec<i32>
    let usize_drone_indices: Vec<usize> = drone_indices.to_vec().into_iter().map(|element| element as usize).collect();
    hive.set_drone_indices(usize_drone_indices);

    let mut packets = rlbot.packeteer();
    loop {
        let packet = packets.next()?;
        let inputs = hive.tick(&packet);

        match inputs.len().cmp(&drone_indices.len()) {
            Ordering::Equal => {
                for i in 0..drone_indices.len() {
                    // first input corresponds to first index in drone_indices, etc.
                    let drone = drone_indices[i];
                    let input = &inputs[i];
                    rlbot.update_player_input(drone, input);
                }
            },
            Ordering::Less => return Err(Box::<dyn Error>::from("too few inputs for drones")),
            Ordering::Greater => return Err(Box::<dyn Error>::from("too many inputs for drones"))
        }
    }
}

fn parse_framework_args() -> Result<Option<HiveFrameworkArgs>, ()> {
    parse_framework_command_line(env::args().skip(1))
}

fn parse_framework_command_line(
    mut args: impl Iterator<Item = String>,
) -> Result<Option<HiveFrameworkArgs>, ()> {

    if args.next().as_ref().map(|s| &s[..]) != Some("--rlbot-version") {
        return Ok(None); // not launched by the framework
    }
    let rlbot_version = args.next().ok_or(())?;

    if args.next().as_ref().map(|s| &s[..]) != Some("--rlbot-dll-directory") {
        return Err(());
    }
    let rlbot_dll_directory = PathBuf::from(args.next().ok_or(())?);

    if args.next().as_ref().map(|s| &s[..]) != Some("--drone-indices") {
        return Err(());
    }
    // Understand the rest of the arguments as drone indices.
    let mut drone_indices: Vec<i32> = vec![];
    for arg in args {
        let index: i32 = arg.parse().map_err(|_| ())?;
        drone_indices.push(index);
    }
    

    Ok(Some(HiveFrameworkArgs {
        rlbot_version,
        rlbot_dll_directory,
        drone_indices,
        _non_exhaustive: (),
    }))
}

// The arguments passed by the RLBot framework for a hivemind.
struct HiveFrameworkArgs {
    // The version of the RLBot framework used to launch the app. This is the
    // same as the version shown when you run this Python code:
    //
    // ```python
    // import rlbot
    // print(rlbot.__version__)
    // ```
    pub rlbot_version: String,

    // The directory containing `RLBot_Core_Interface.dll` and
    // `RLBot_Injector.exe`.
    pub rlbot_dll_directory: PathBuf,

    // The indices of the bots you're controlling
    pub drone_indices: Vec<i32>,

    _non_exhaustive: (),
}

impl From<HiveFrameworkArgs> for InitOptions {
    fn from(args: HiveFrameworkArgs) -> Self {
        Self::new().rlbot_dll_directory(args.rlbot_dll_directory)
    }
}