use os_foundry_suite::prelude::*;

fn main() -> Result<()> {
    let mut blueprint = Profiles::dev_qemu();
    blueprint.name = "example-os";

    let report = os_foundry_suite::validate_blueprint_integrations(&blueprint);
    if !report.is_ok() {
        return Err(Error::invalid_blueprint(
            report.errors.first().copied().unwrap_or("unknown error"),
        ));
    }

    let plan = OsBuilder::new(blueprint)
        .target(Target::x86_64_bare_metal())
        .validate_and_plan()?;

    println!(
        "Plan: arch={:?}, bare_metal={}, uefi={}",
        plan.target.arch, plan.target.bare_metal, plan.target.uefi
    );

    Ok(())
}
