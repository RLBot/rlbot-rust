use crate::{
    game::{ControllerState, GameTickPacket},
    init_with_options, InitOptions,
};
use std::{cmp::Ordering, env, error::Error, path::PathBuf};

// Most of this is basically a copy of what is in framework.rs,
// just adapted for hivemind.

/// Your hivemind bot should implement this trait.
/// Use this like you would use the Bot trait.
///
/// The drone_indices are indices of the bots you control.
pub trait Hivemind {
    /// This method is called after your hivemind has been passed into
    /// `run_hive()`. It gives you the a vector of bot indices which
    /// correspond to those which you control.
    fn set_drone_indices(&mut self, indices: Vec<usize>);

    /// This is called whenever there is a new game state. Your cars will be
    /// controlled according to the inputs you return.
    ///
    /// The first input will go to the first index in `drone_indices`, the
    /// second to the second, etc.
    fn tick(&mut self, packet: &GameTickPacket) -> Vec<ControllerState>;
}

/// Runs your hivemind bot. Pass in your struct which implements the Hivemind
/// trait.
///
/// Calls `tick()` on your hivemind and expects a Vec<ControllerState>. The
/// first element corresponds to the first index in `drone_indices`, the second
/// to the second, etc.
pub fn run_hive<H: Hivemind>(mut hive: H) -> Result<(), Box<dyn Error>> {
    let args = parse_hive_framework_args()
        .map_err(|_| Box::<dyn Error>::from("could not parse framework arguments"))?
        .ok_or_else(|| Box::<dyn Error>::from("not launched by framework"))?;

    let drone_indices = args.drone_indices.clone();
    let rlbot = init_with_options(args.into())?;

    // Create a Vec<usize> copy of the original Vec<i32>.
    let usize_drone_indices: Vec<usize> = drone_indices
        .clone()
        .into_iter()
        .map(|element| element as usize)
        .collect();
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
                    rlbot.update_player_input(drone, input)?;
                }
            }
            Ordering::Less => return Err(Box::<dyn Error>::from("too few inputs for drones")),
            Ordering::Greater => return Err(Box::<dyn Error>::from("too many inputs for drones")),
        }
    }
}

// Same as in framework.rs except it's for a hivemind.
/// Parse the arguments passed by the RLBot framework.
///
/// This function returns:
///
/// * `Ok(Some(args))` – if the app was launched by the framework.
/// * `Ok(None)` – if the app was *not* launched by the framework.
/// * `Err(_)` – if it appears the app was launched by the framework, but we
///   could not understand the arguments.
pub fn parse_hive_framework_args() -> Result<Option<HiveFrameworkArgs>, ()> {
    parse_framework_command_line(env::args().skip(1))
}

// Same as in framework.rs except it's for a hivemind.
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

/// The arguments passed by the RLBot framework for a hivemind.
pub struct HiveFrameworkArgs {
    /// The version of the RLBot framework used to launch the app. This is the
    /// same as the version shown when you run this Python code:
    ///
    /// ```python
    /// import rlbot
    /// print(rlbot.__version__)
    /// ```
    pub rlbot_version: String,

    /// The directory containing `RLBot_Core_Interface.dll` and
    /// `RLBot_Injector.exe`.
    pub rlbot_dll_directory: PathBuf,

    /// The indices of the bots you are controlling.
    pub drone_indices: Vec<i32>,

    _non_exhaustive: (),
}

impl From<HiveFrameworkArgs> for InitOptions {
    fn from(args: HiveFrameworkArgs) -> Self {
        Self::new().rlbot_dll_directory(args.rlbot_dll_directory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pfcl(ss: Vec<&str>) -> Result<Option<HiveFrameworkArgs>, ()> {
        parse_framework_command_line(ss.into_iter().map(str::to_string))
    }

    #[test]
    fn parse_hive_framework_args() {
        let args = pfcl(vec![
            "--rlbot-version",
            "1.35.5",
            "--rlbot-dll-directory",
            "/tmp",
            "--drone-indices",
            "0",
            "1",
            "2",
            "3",
            "4",
            "5",
        ])
        .unwrap()
        .unwrap();
        assert_eq!(args.rlbot_version, "1.35.5");
        assert_eq!(args.rlbot_dll_directory.to_str().unwrap(), "/tmp");
        assert_eq!(args.drone_indices, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn parse_empty_command_line() {
        let args = pfcl(vec![]).unwrap();
        assert!(args.is_none());
    }

    #[test]
    fn parse_non_matching_command_line() {
        let args = pfcl(vec!["--unrelated-argument"]).unwrap();
        assert!(args.is_none());
    }

    #[test]
    fn parse_error() {
        let args = pfcl(vec!["--rlbot-version"]);
        assert!(args.is_err());

        let args = pfcl(vec!["--rlbot-version", "1.35.5"]);
        assert!(args.is_err());
    }
}
