fn main() {
    let devices = [
        "aplite", "basalt", "chalk", "diorite", "emery", "flint", "gabbro",
    ];
    println!(
        "cargo::rustc-check-cfg=cfg(target_device, values({}))",
        format!("{devices:?}").trim_matches(['[', ']']),
    );

    println!("cargo::rerun-if-env-changed=TARGET_DEVICE");

    let target_device = std::env::var("TARGET_DEVICE").unwrap_or_else(|_| "emery".into());
    println!(r#"cargo::rustc-cfg=target_device="{target_device}""#)
}
