use std::io;
use std::panic;
use std::process::Command;
use std::thread;
use std::time::Duration;
use winapi::um::processthreadsapi::TerminateProcess;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winbase::WAIT_OBJECT_0;
use winproc::Process;

/// Runs a controlled copy of Rocket League alongside a closure. It will start
/// Rocket League before the closure runs, and terminate Rocket League after
/// the closure returns. In case of an unwind/panic, Rocket League will be
/// terminated as normal, and then the unwind will resume.
pub fn with_rocket_league<R>(f: impl FnOnce() -> R + panic::UnwindSafe) -> R {
    if find_rocket_league().is_some() {
        panic!("Rocket League is already running");
    }

    // Sorry if this path isn't the same on your system.
    let path = r"C:\Program Files (x86)\Steam\steamapps\common\rocketleague\Binaries\Win32\RocketLeague.exe";
    Command::new(path).spawn().unwrap();
    thread::sleep(Duration::from_secs(15));

    let result = panic::catch_unwind(f);

    find_rocket_league().unwrap().terminate(1).unwrap();

    match result {
        Ok(result) => result,
        Err(panic) => panic::resume_unwind(panic),
    }
}

fn find_rocket_league() -> Option<Process> {
    Process::all()
        .unwrap()
        .find(|p| p.name().unwrap() == "RocketLeague.exe")
}

trait TerminateProcess {
    fn terminate(&self, exit_code: u32) -> Result<(), io::Error>;
}

impl TerminateProcess for Process {
    fn terminate(&self, exit_code: u32) -> Result<(), io::Error> {
        match unsafe { TerminateProcess(**self.handle(), exit_code) } {
            0 => return Err(io::Error::last_os_error()),
            _ => {}
        }

        match unsafe { WaitForSingleObject(**self.handle(), 15_000) } {
            WAIT_OBJECT_0 => Ok(()),
            _ => Err(io::Error::last_os_error()),
        }
    }
}
