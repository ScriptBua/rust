use crate::spec::{LinkerFlavor, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = super::vxworks_base::opts();
    base.max_atomic_width = Some(128);

    Target {
        llvm_target: "aarch64-unknown-linux-gnu".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        target_c_int_width: "32".to_string(),
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".to_string(),
        arch: "aarch64".to_string(),
        target_os: "vxworks".to_string(),
        target_env: "gnu".to_string(),
        target_vendor: "wrs".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        options: TargetOptions { unsupported_abis: super::arm_base::unsupported_abis(), ..base },
    }
}
