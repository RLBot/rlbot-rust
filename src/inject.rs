//! This module handles injecting the core DLL, using RLBot's provided
//! injector. It is basically a reimplementation of
//! https://github.com/RLBot/RLBot/blob/928d0b1660618ef2c88b8aaf218189e8fb6b744b/src/main/python/rlbot/utils/structures/game_interface.py#L175

use crate::utils::maybe_join;
use std::{error::Error, fmt, mem, path::Path, process::Command, thread::sleep, time::Duration};

pub fn inject_dll(rlbot_dll_directory: Option<&Path>) -> Result<InjectorCode, Box<Error>> {
    let program = maybe_join(rlbot_dll_directory, "RLBot_Injector");
    let code = Command::new(program)
        .arg("hidden")
        .status()?
        .code()
        .unwrap(); // There will always be an exit code.

    let code: InjectorCode = unsafe { mem::transmute(code) };
    match code {
        InjectorCode::InjectionSuccessful => {
            // If rlbot is freshly injected, give it some time to sink its hooks in.
            sleep(Duration::from_secs(20));
            Ok(code)
        }
        InjectorCode::RLBotDLLAlreadyInjected => Ok(code),
        _ => Err(From::from(code)),
    }
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum InjectorCode {
    InjectionSuccessful = 0,
    InjectionFailed = 1,
    MultipleRocketLeagueProcessesFound = 2,
    RLBotDLLAlreadyInjected = 3,
    RLBotDLLNotFound = 4,
    MultipleRLBotDLLFilesFound = 5,
}

impl Error for InjectorCode {}

impl fmt::Display for InjectorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(self, f)
    }
}
