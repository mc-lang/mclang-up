use std::{path::{PathBuf, Path}, process::{Command, Stdio}};
use crate::{util::Prompt, info, code_block, help, cmd, warn};
use color_eyre::Result;
use eyre::eyre;

use crate::{Args, error};

const MCLANGC_GIT: &str = "https://github.com/mc-lang/mclangc.git";
const MCLANG_UP_GIT: &str = "https://github.com/mc-lang/mclang-up.git";
const MCLANG_PKM_GIT: &str = "https://github.com/mc-lang/mclang-pkm.git";


pub fn install(args: &Args) -> Result<()> {
    let install_path = get_install_location()?;
    let install_path_str = install_path.clone();
    let install_path_str = install_path_str.to_str().unwrap();
    let install_branch = get_install_branch()?;

    if !Prompt::bool("Are you sure you want to proceed? This may delete your files if you put any in the mclang install location", Some(false))? {
        return Err(eyre!(""));
    }

    info!("Beginning installation of mclang {install_branch} to {install_path:?}");

    info!("Checking dependencies");
    check_if_installed(args,"nasm");
    check_if_installed(args,"git");
    check_if_installed(args,"cargo");

    info!("Creating '{install_path_str}' if it doesnt exist");

    run_cmd(args, PathBuf::from("./"), "mkdir", vec!["-p", install_path_str])?;
    run_cmd(args, PathBuf::from("./"), "mkdir", vec!["-p", format!("{}{}", install_path_str, "/components").as_str()])?;

    info!("Cleaning out old versions");

    run_cmd(args, install_path.join("components"), "rm", vec!["-rf", "./mclangc", "./mclang-up", "./mclang-pkm"])?;
    run_cmd(args, install_path.clone(), "rm", vec!["-rf", "./stdlib"])?;

    info!("Cloning component repositories");

    run_cmd(args, install_path.join("components"), "git", vec!["clone", "-b", &install_branch, MCLANGC_GIT])?;
    run_cmd(args, install_path.join("components"), "git", vec!["clone", "-b", &install_branch, MCLANG_UP_GIT])?;
    run_cmd(args, install_path.join("components"), "git", vec!["clone", "-b", &install_branch, MCLANG_PKM_GIT])?;
    
    info!("Compiling mclangc");
    run_cmd(args, install_path.join("components/mclangc"), "cargo", vec!["build", "--release"])?;
    info!("Compiling mclang-up");
    run_cmd(args, install_path.join("components/mclang-up"), "cargo", vec!["build", "--release"])?;
    info!("Compiling mclang-pkm");
    run_cmd(args, install_path.join("components/mclang-pkm"), "cargo", vec!["build", "--release"])?;

    info!("Creating '{install_path_str}/bin'");
    run_cmd(args, install_path.clone(), "mkdir", vec!["-p", ",/bin"])?;
    
    info!("Copying binaries to '{install_path_str}/bin'");
    run_cmd(args, install_path.clone(), "cp", vec!["-f", 
        "./components/mclangc/target/release/mclangc",
        "./components/mclang-up/target/release/mclang-up",
        "./components/mclang-pkm/target/release/mclang-pkm",
        "./bin"
    ])?;
    
    info!("Creating '{install_path_str}/stdlib'");
    run_cmd(args, install_path.clone(), "mkdir", vec!["-p", "./stdlib"])?;

    info!("Copying standart lib to '{install_path_str}/stdlib'");

    
    copy_dir_all(format!("{install_path_str}/components/mclangc/include"), format!("{install_path_str}/stdlib")).unwrap();

    warn!("Before you can use MCLang you have to put 'export PATH=\"$PATH:{install_path_str}/bin\"' in your .bashrc or .zshrc (for fish shell it is diffrent)");

    info!("MCLang was successfully installed");
    Ok(())
}

pub fn update(args: &Args) -> Result<()> {
    
    let install_path = get_install_location()?;
    let install_path_str = install_path.clone();
    let install_path_str = install_path_str.to_str().unwrap();
    let install_branch = get_install_branch()?;
    if !Prompt::bool("Are you sure you want to proceed? This may delete your files if you put any in the mclang install location", Some(false))? {
        return Err(eyre!(""));
    }
    info!("Beginning update of mclang {install_branch} to {install_path:?}");

    info!("Checking dependencies");
    check_if_installed(args,"nasm");
    check_if_installed(args,"git");
    check_if_installed(args,"cargo");


    info!("Cloning component repositories");

    run_cmd(args, install_path.join("components/mclangc"), "git", vec!["pull", "origin", &install_branch])?;
    run_cmd(args, install_path.join("components/mclang-up"), "git", vec!["pull", "origin", &install_branch])?;
    run_cmd(args, install_path.join("components/mclang-pkm"), "git", vec!["pull", "origin", &install_branch])?;
    
    info!("Compiling mclangc");
    run_cmd(args, install_path.join("components/mclangc"), "cargo", vec!["build", "--release"])?;
    info!("Compiling mclang-up");
    run_cmd(args, install_path.join("components/mclang-up"), "cargo", vec!["build", "--release"])?;
    info!("Compiling mclang-pkm");
    run_cmd(args, install_path.join("components/mclang-pkm"), "cargo", vec!["build", "--release"])?;

    
    info!("Copying binaries to '{install_path_str}/bin'");
    run_cmd(args, install_path.clone(), "cp", vec!["-f", 
        "./components/mclangc/target/release/mclangc",
        "./components/mclang-up/target/release/mclang-up",
        "./components/mclang-pkm/target/release/mclang-pkm",
        "./bin"
    ])?;
    
    info!("Copying standart lib to '{install_path_str}/stdlib'");
    
    copy_dir_all(format!("{install_path_str}/components/mclangc/include"), format!("{install_path_str}/stdlib")).unwrap();

    info!("MCLang was successfully updated");
    Ok(())
}

pub fn check_if_installed(args: &Args, prog: &str) {
    if std::process::Command::new(prog)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn().is_ok() {
        if args.verbose {
            info!("Program {prog:?} was found.");
        }
    } else {
        error!("Program {prog:?} could not be found.");
        if prog == "cargo" {
            help!("To install the rust toolchain folow the instructions on https://rustup.rs/");
            help!("Or run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`")
        } else {   
            help!("Install {prog:?} with your distributions package manager.");
            help!("    Ubuntu/Debian: sudo apt install {prog}");
            help!("    Arch: sudo pacman -Sy {prog}");
        }
    }
}

pub fn run_cmd(args: &Args, cwd: PathBuf, prog: &str, cmd: Vec<&str>) -> Result<()>{
    if args.verbose {
        cmd!("Running '{prog} {}'", cmd.join(" "));
    }

    let mut command = Command::new(prog);
    let command = command.args(cmd);
    let mut command = command.current_dir(cwd);
    if args.verbose {
        command = command
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
    } else {
        command = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
    }
    let child = command.spawn()?;

    let out = child.wait_with_output()?;
    match out.status.code() {
        Some(code) => {
            if code != 0 {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);
                
                if !stdout.is_empty() {
                    cmd!("STDOUT:\n{}", code_block!("{}", stdout));
                }
                if !stderr.is_empty() {
                    cmd!("STDERR:\n{}", code_block!("{}", stderr));
                }
                cmd!("Exited with status code: {code}");
                return Err(eyre!(""));
            } else {
                if args.verbose {
                    cmd!("Exited with status code: {code}")
                }
            }
        },
        None => {
            cmd!("Process terminated by signal");
            return Err(eyre!(""));
        }
    }
    Ok(())
}


fn get_install_location() -> Result<PathBuf>{
    
    let p = Prompt::default("Enter install location", format!("{}/.mclang", env!("HOME")).as_str())?;

    Ok(PathBuf::from(p))
}

fn get_install_branch() -> Result<String>{
    
    let p = Prompt::default("Enter install branch, 'dev' or 'stable'", "stable")?;
    match p.as_str() {
        "dev" | "stable" => (),
        s => {
            error!("Unknown value {s:?}, please answer 'dev' or 'stable'");
            return Err(eyre!(""));
        }
    }
    Ok(p)
}


fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}