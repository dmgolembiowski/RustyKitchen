#[macro_use]
extern crate error_chain;
extern crate semver;

use std::process::Command;
use semver::{Version, VersionReq};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Utf8(std::string::FromUtf8Error);
        SemVer(semver::SemVerError);
        SemVerReq(semver::ReqParseError);
    }
}

fn run() -> Result<()> {
    let version_constraint = "> 1.12.0";
    let version_test = VersionReq::parse(version_constraint)?;
    let output = Command::new("git").arg("--version").output()?;

    if !output.status.success() {
        bail!("Command executed with failing error code");
    }

    let stdout = String::from_utf8(output.stdout)?;
    let version = stdout.split(" ").last().ok_or_else(|| {
        "Invalid command output"
    })?;
    let parsed_version = Version::parse(version)?;

    if !version_test.matches(&parsed_version) {
        bail!("Command version lower than minimum supported version (found {}, need {})",
            parsed_version, version_constraint);
    }

    Ok(())
}

quick_main!(run);

