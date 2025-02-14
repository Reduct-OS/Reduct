use argh::FromArgs;
use builder::ImageBuilder;
use ovmf_prebuilt::{Arch, FileType, Prebuilt, Source};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(FromArgs)]
#[argh(description = "ReductOS bootloader and kernel builder")]
struct Args {
    #[argh(switch, short = 'b')]
    #[argh(description = "boot the constructed image")]
    boot: bool,

    #[argh(switch, short = 'k')]
    #[argh(description = "use KVM acceleration")]
    kvm: bool,

    #[argh(switch, short = 'w')]
    #[argh(description = "use Hyper-V acceleration")]
    whpx: bool,

    #[argh(option, short = 'c')]
    #[argh(default = "4")]
    #[argh(description = "number of CPU cores")]
    cores: usize,

    #[argh(switch, short = 's')]
    #[argh(description = "redirect serial to stdio")]
    serial: bool,
}

fn main() {
    let img_path = build_img();
    let args: Args = argh::from_env();

    if args.boot {
        let mut cmd = Command::new("qemu-system-x86_64");

        let ovmf_path = Prebuilt::fetch(Source::LATEST, "target/ovmf")
            .expect("failed to update prebuilt")
            .get_file(Arch::X64, FileType::Code);
        let ovmf_config = format!("if=pflash,format=raw,file={}", ovmf_path.display());

        cmd.arg("-machine").arg("q35");
        cmd.arg("-drive").arg(ovmf_config);
        cmd.arg("-m").arg("4096");
        cmd.arg("-smp").arg(format!("cores={}", args.cores));
        cmd.arg("-cpu").arg("qemu64");

        let drive_config = format!("if=none,format=raw,id=disk0,file={}", img_path.display());
        cmd.arg("-device").arg("nvme,drive=disk0,serial=HARDDISK");
        cmd.arg("-drive").arg(drive_config);

        if args.kvm {
            cmd.arg("--enable-kvm");
        }
        if args.whpx {
            cmd.arg("-accel").arg("whpx");
        }
        if args.serial {
            cmd.arg("-serial").arg("stdio");
        }

        let mut child = cmd.spawn().unwrap();
        child.wait().unwrap();
    }
}

fn build_img() -> PathBuf {
    let kernel_path = Path::new(env!("CARGO_BIN_FILE_KERNEL"));
    println!("Building UEFI disk image for kernel at {:#?}", &kernel_path);

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let assets_dir = manifest_dir.join("assets");

    let files = BTreeMap::from([
        ("kernel", kernel_path.to_path_buf()),
        ("efi/boot/bootx64.efi", assets_dir.join("BOOTX64.EFI")),
        ("limine.conf", assets_dir.join("limine.conf")),
        (
            "drv/acpid",
            Path::new(env!("CARGO_BIN_FILE_ACPID")).to_path_buf(),
        ),
        (
            "drv/pcid",
            Path::new(env!("CARGO_BIN_FILE_PCID")).to_path_buf(),
        ),
        (
            "drv/nvmed",
            Path::new(env!("CARGO_BIN_FILE_NVMED")).to_path_buf(),
        ),
        (
            "drv/fsmd",
            Path::new(env!("CARGO_BIN_FILE_FSMD")).to_path_buf(),
        ),
    ]);

    let img_path = manifest_dir.parent().unwrap().join("ReductOS.img");
    ImageBuilder::build(files, &img_path).expect("Failed to build UEFI disk image");
    println!("Created bootable UEFI disk image at {:#?}", &img_path);

    img_path
}
