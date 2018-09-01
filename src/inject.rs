//! This module handles injecting the core DLL, using RLBot's provided
//! injector. It is basically a reimplementation of
//! https://github.com/RLBot/RLBot/blob/fa959462f8a54752afc64769cf141db904efae8a/src/main/python/rlbot/utils/structures/game_interface.py#L165

use std::error::Error;
use std::fmt;
use std::mem;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub fn inject_dll() -> Result<InjectorCode, Box<Error>> {
    let code = Command::new("RLBot_Injector")
        .arg("hidden")
        .status()?
        .code()
        .unwrap(); // There will always be an exit code.

    let code: InjectorCode = unsafe { mem::transmute(code) };
    match code {
        InjectorCode::InjectionSuccessful => {
            // If rlbot is freshly injected, give it some time to sink its hooks in.
            sleep(Duration::from_secs(11));
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
