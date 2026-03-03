use os_foundry_suite::prelude::*;

fn main() -> Result<()> {
    let blueprint = Profiles::uefi_dev();
    let target = Target::x86_64_uefi();

    let plan = plan_image(&blueprint, target)?.expect("expected an image plan");

    println!("Image kind: {:?}", plan.kind);
    println!("Partition scheme: {:?}", plan.layout.scheme);
    println!("Boot artifact: {:?}", plan.layout.boot);
    println!("Partitions:");

    for p in plan.layout.partitions {
        println!("- {} ({} MiB)", p.label, p.size_mib);
    }

    Ok(())
}
