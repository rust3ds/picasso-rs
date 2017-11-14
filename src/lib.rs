extern crate failure;
#[macro_use] extern crate failure_derive;

use std::io;
use std::ffi::OsStr;
use std::process::Command;

pub fn assemble<I, O>(source_paths: &[I], out_path: O) -> Result<()> where
    I: AsRef<OsStr>,
    O: AsRef<OsStr>,
{
    let output = Command::new("picasso")
        .arg("-o").arg(out_path)
        .arg("--")
        .args(source_paths)
        .output()
        .map_err(Error::Exec)?;
    
    if !output.status.success() {
        let msg = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(Error::Msg(msg));
    }

    Ok(())
}

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug,Fail)]
pub enum Error {
    #[fail(display="Failed to run `picasso` ({})", _0)]
    Exec(io::Error),
    #[fail(display="`picasso` exited with errors:\n{}", _0)]
    Msg(String),
}
