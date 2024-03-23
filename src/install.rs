use std::{path::{PathBuf, Path}, process::{Command, Stdio}};
use color_eyre::Result;
use eyre::eyre;

use crate::{util::Prompt, Args};


const MCLANGC_GIT: &str = "https://github.com/mc-lang/mclangc.git";
const MCLANG_UP_GIT: &str = "https://github.com/mc-lang/mclang-up.git";
const MCLANG_PKM_GIT: &str = "https://github.com/mc-lang/mclang-pkm.git";
const LIBMC_GIT: &str = "https://github.com/mc-lang/libmc.git";


pub fn install(args: &Args) -> Result<()> {
    let install_path = get_install_location()?;
    let install_path_str = install_path.clone();
    let install_path_str = install_path_str.to_str().unwrap();
    let install_branch = get_install_branch()?;

    if !Prompt::bool("Are you sure you want to proceed? This may delete your files if you put any in the mclang install location", Some(false))? {
        return Err(eyre!(""));
    }

    log::info!("Beginning installation of mclang {install_branch} to {install_path:?}");

    log::info!("Checking dependencies");
    check_if_installed(args,"git");
    check_if_installed(args,"cargo");

    log::info!("Creating '{install_path_str}' if it doesnt exist");

    run_cmd(args, PathBuf::from("./"), "mkdir", vec!["-p", install_path_str])?;
    run_cmd(args, PathBuf::from("./"), "mkdir", vec!["-p", format!("{}{}", install_path_str, "/components").as_str()])?;

    log::info!("Cleaning out old versions");
    run_cmd(args, install_path.join("components"), "rm", vec!["-rf", "./mclangc", "./mclang-up", "./mclang-pkm", "./libmc"])?;

    log::info!("Cloning component repositories");

    install_component(args, &install_path, &install_branch, MCLANGC_GIT, "mclangc")?;
    install_component(args, &install_path, &install_branch, MCLANG_UP_GIT, "mclang-up")?;
    install_component(args, &install_path, &install_branch, MCLANG_PKM_GIT, "mclang-pkm")?;
    log::info!("Cloning libmc");
    run_cmd(args, install_path.join("components"), "git", vec!["clone", "-b", &install_branch, LIBMC_GIT])?;

    log::info!("Creating '{install_path_str}/bin'");
    run_cmd(args, install_path.clone(), "mkdir", vec!["-p", "./bin"])?;
    
    log::info!("Copying binaries to '{install_path_str}/bin'");
    run_cmd(args, install_path.clone(), "cp", vec!["-f", 
        "./components/mclangc/target/release/mclangc",
        "./components/mclang-up/target/release/mclang-up",
        "./components/mclang-pkm/target/release/mclang-pkm",
        "./bin"
    ])?;

    log::warn!("Before you can use MCLang you have to put 'export PATH=\"$PATH:{install_path_str}/bin\"' in your .bashrc or .zshrc (for fish shell it is diffrent)");

    log::info!("MCLang was successfully installed");
    Ok(())
}

fn install_component(args: &Args, install_path: &PathBuf, branch: &String, git_url: &'static str, name: &'static str) -> Result<()>{
    log::info!("Cloning {name}");
    run_cmd(args, install_path.join("components"), "git", vec!["clone", "-b", &branch, git_url])?;
    log::info!("Building {name}");
    run_cmd(args, install_path.join(format!("components/{name}")), "cargo", vec!["build", "--release"])?;
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
    log::info!("Beginning update of mclang {install_branch} to {install_path:?}");

    log::info!("Checking dependencies");
    check_if_installed(args,"git");
    check_if_installed(args,"cargo");


    update_component(args, &install_path, &install_branch, "mclangc")?;
    update_component(args, &install_path, &install_branch, "mclang-up")?;
    update_component(args, &install_path, &install_branch, "mclang-pkm")?;
    log::info!("Installing libmc");
    run_cmd(args, install_path.join(format!("components/libmc")), "git", vec!["pull", "origin", &install_branch])?;


    
    log::info!("Copying binaries to '{install_path_str}/bin'");
    run_cmd(args, install_path.clone(), "cp", vec!["-f", 
        "./components/mclangc/target/release/mclangc",
        "./components/mclang-up/target/release/mclang-up",
        "./components/mclang-pkm/target/release/mclang-pkm",
        "./bin"
    ])?;
    

    log::info!("MCLang was successfully updated");
    Ok(())
}

fn update_component(args: &Args, install_path: &PathBuf, branch: &String, name: &'static str) -> Result<()>{
    log::info!("Updating {name}");
    run_cmd(args, install_path.join(format!("components/{name}")), "git", vec!["pull", "origin", &branch])?;
    log::info!("Building {name}");
    run_cmd(args, install_path.join(format!("components/{name}")), "cargo", vec!["build", "--release"])?;
    Ok(())
}

pub fn check_if_installed(args: &Args, prog: &str) {
    if std::process::Command::new(prog)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn().is_ok() {
        if args.verbose {
            log::info!("Program {prog:?} was found.");
        }
    } else {
        log::error!("Program {prog:?} could not be found.");
        if prog == "cargo" {
            log::info!("To install the rust toolchain folow the instructions on https://rustup.rs/");
            log::info!("Or run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`")
        } else {   
            log::info!("Install {prog:?} with your distributions package manager.");
            log::info!("    Ubuntu/Debian: sudo apt install {prog}");
            log::info!("    Arch: sudo pacman -Sy {prog}");
        }
    }
}

pub fn run_cmd(args: &Args, cwd: PathBuf, prog: &str, cmd: Vec<&str>) -> Result<()>{
    if args.verbose {
        log::info!("Running '{prog} {}'", cmd.join(" "));
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
                    log::info!("STDOUT:\n{}", crate::util::code_block(format!("{}", stdout).as_str()));
                }
                if !stderr.is_empty() {
                    log::info!("STDERR:\n{}", crate::util::code_block(format!("{}", stderr).as_str()));
                }
                log::info!("Exited with status code: {code}");
                return Err(eyre!(""));
            } else {
                if args.verbose {
                    log::info!("Exited with status code: {code}")
                }
            }
        },
        None => {
            log::info!("Process terminated by signal");
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
        "dev" | "stable" | "main-v2" => (),
        s => {
            log::error!("Unknown value {s:?}, please answer 'dev' or 'stable'");
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