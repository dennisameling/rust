use crate::spec::{LinkerFlavor, Target};

pub fn target() -> Target {
    let mut base = super::windows_gnu_base::opts();
    let gcc_pre_link_args = base.pre_link_args.entry(LinkerFlavor::Gcc).or_default();
    gcc_pre_link_args.push("-march=armv8-a".to_string());
    base.max_atomic_width = Some(64);
    base.linker = Some("aarch64-w64-mingw32-clang".to_string());

    Target {
        llvm_target: "aarch64-pc-windows-gnu".to_string(),
        pointer_width: 64,
        data_layout: "e-m:w-p:64:64-i32:32-i64:64-i128:128-n32:64-S128"
            .to_string(),
        arch: "aarch64".to_string(),
        options: base,
    }
}
