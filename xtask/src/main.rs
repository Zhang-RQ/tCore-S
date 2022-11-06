mod pack;

use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use clap::{Args, Parser, Subcommand};
use once_cell::sync::Lazy;
use pack::easy_fs_pack;

const DEFAULT_KERNEL: &str = "tcore-kernel";
const DEFAULT_ARCH: &str = "riscv64";
const DEFAULT_TARGET: &str = "riscv64gc-unknown-none-elf";

static PROJECT: Lazy<&'static Path> =
    Lazy::new(|| Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap());
static TARGET: Lazy<PathBuf> = Lazy::new(|| PROJECT.join("target").join(DEFAULT_TARGET));

const LOCAL_TESTCASES: &'static [&'static str] = &["hello_world"];
static EASY_FS_IMG: Lazy<PathBuf> = Lazy::new(|| PROJECT.join("easy-fs.img"));

#[derive(Parser)]
#[clap(name = "tCore")]
#[clap(version, about, long_about = None)]
struct Commands {
    #[clap(subcommand)]
    inner: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    Make(BuildArgs),
    Qemu(QemuArgs),
}

/// Main build arguments for this project
#[derive(Args, Default)]
struct BuildArgs {
    /// Kernel package name
    #[clap(long, default_value = DEFAULT_KERNEL)]
    kernel: Option<String>,

    /// Build architecture
    #[clap(long, default_value = DEFAULT_ARCH)]
    arch: Option<String>,

    /// Build target
    #[clap(long, default_value = DEFAULT_TARGET)]
    target: Option<String>,

    /// Run platform
    #[clap(long, default_value = "qemu")]
    plat: Option<String>,

    /// Choose log level
    #[clap(long, default_value = "debug")]
    log: Option<String>,

    /// Start test instead of normal build
    #[clap(long)]
    test: bool,

    /// Choose optimizing level
    #[clap(long)]
    debug: bool,

    /// Dump binary file to ASM
    #[clap(long)]
    dump: bool,

    /// Use global or local user tests
    #[clap(long)]
    global: bool,
}

impl BuildArgs {
    fn make(&self) -> PathBuf {
        // Prepare cargo utils
        Command::new("cargo")
            .args(&["install", "cargo-binutils"])
            .status()
            .expect("Failed to install cargo-binutils");
        Command::new("rustup")
            .args(&["component", "add", "rust-src", "llvm-tools-preview"])
            .status()
            .expect("Failed to add components");

        // Debug mode, Release mode and Test mode
        let target = self.target.as_ref().unwrap().as_str();
        let opt_level = if self.debug {
            "--profile=dev"
        } else {
            "--release"
        };
        let (subcmd, options) = if self.test {
            ("rustc", "-- --test")
        } else {
            ("build", opt_level)
        };
        let features = if self.global {
            "global_test"
        } else {
            let user = PROJECT.join("user");
            let user_root = user.to_str().unwrap();
            let user_src = format!("{}/src", &user_root);
            let user_target = format!("{}/target", &user_root);

            // Build all user testcases
            Command::new("cargo")
                .arg("build")
                .arg("--quiet")
                .args(&["--package", "user_lib"])
                .args(&["--target", target])
                .arg("--release")
                .env("CARGO_TARGET_DIR", &user_target)
                .env(
                    "RUSTFLAGS",
                    format!("-Clink-arg=-T{}", format!("{}/linker.ld", user_src)),
                )
                .status()
                .expect("Failed to run cargo");

            // Build easy_fs image form local testcase list
            let mut cases: Vec<&str> = Vec::new();
            cases.extend(LOCAL_TESTCASES.into_iter());
            easy_fs_pack(
                &cases,
                format!(
                    "{}/{}/{}",
                    &user_target,
                    target,
                    if self.debug { "debug" } else { "release" }
                )
                .as_str(),
                EASY_FS_IMG.as_os_str().to_str().unwrap(),
            )
            .expect("Faild to pack user tests");
            "local_test"
        };

        // Linker file for target platform to configure kernel layout
        let linker = PROJECT
            .join("plat")
            .join(self.plat.as_ref().unwrap())
            .join("linker.ld");

        // Run cargo build command
        Command::new("cargo")
            .arg(subcmd)
            .args(&["--package", self.kernel.as_ref().unwrap().as_str()])
            .args(&["--target", target])
            .args(&["--features", features])
            .arg(options)
            .env("LOG", self.log.as_ref().unwrap().as_str())
            .env(
                "RUSTFLAGS",
                format!("-Clink-arg=-T{}", linker.as_os_str().to_str().unwrap()),
            )
            .status()
            .expect("Failed to run cargo");
        let kernel = TARGET
            .join(if self.debug { "debug" } else { "release" })
            .join(self.kernel.as_ref().unwrap());

        // Dump ASM file
        if self.dump {
            let kernel_asm = kernel.with_extension("S");
            let asm = Command::new("rust-objdump")
                .arg(format!(
                    "--arch-name={}",
                    self.arch.as_ref().unwrap().as_str()
                ))
                .args(&["-d"])
                .arg(&kernel)
                .output()
                .expect("Failed to dump kernel ASM");
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(kernel_asm.as_os_str().to_str().unwrap())
                .expect("Failed to open or create target asm file");
            file.write(String::from_utf8_lossy(&asm.stdout).as_bytes())
                .expect("Failed to write to target asm file");
        }
        kernel
    }
}

/// Run on Qemu
#[derive(Args)]
struct QemuArgs {
    /// Use Build Arguments
    #[clap(flatten)]
    build: BuildArgs,

    /// Using gdb
    #[clap(long)]
    gdb: bool,
}

impl QemuArgs {
    fn run(&self) {
        // Build the kernel ELF
        assert!(self.build.plat.as_ref().unwrap().eq("qemu"));
        let kernel = self.build.make();

        // Kernel binary for qemu
        let kernel_bin = kernel.with_extension("bin");
        Command::new("rust-objcopy")
            .arg(format!(
                "--binary-architecture={}",
                self.build.arch.as_ref().unwrap().as_str()
            ))
            .arg(&kernel)
            .args(&["--strip-all", "-O", "binary"])
            .arg(&kernel_bin)
            .status()
            .expect("Failed to generate kernel binary file");

        // RustSBI bootloader binary for qemu
        let bootloader = PROJECT.join("plat/qemu/rustsbi.bin");

        // Run Qemu
        let mut cmd = Command::new(format!(
            "qemu-system-{}",
            self.build.arch.as_ref().unwrap().as_str()
        ));
        cmd.args(&["-machine", "virt"])
            .args(&["-m", "2G"])
            .arg("-nographic")
            .arg("-bios")
            .arg(&bootloader)
            .arg("-kernel")
            .arg(&kernel_bin)
            .args(&["-serial", "mon:stdio"]);
        if !self.build.global {
            cmd.args(&[
                "-drive",
                format!(
                    "file={},if=none,format=raw,id=x0",
                    if self.build.global {
                        String::new()
                    } else {
                        PROJECT
                            .join("easy-fs.img")
                            .into_os_string()
                            .into_string()
                            .unwrap()
                    }
                )
                .as_str(),
                "-device",
                "virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0",
            ]);
        }
        if self.gdb {
            cmd.args(&["-s", "-S"]);
        }
        cmd.status().expect("Failed to run qemu");
    }
}

fn main() {
    match Commands::parse().inner {
        Subcommands::Make(args) => {
            args.make();
        }
        Subcommands::Qemu(args) => args.run(),
    }
}
