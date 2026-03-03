/// CPU architecture descriptor used by [`Target`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Arch {
    /// x86_64.
    X86_64,
    /// AArch64.
    Aarch64,
}

/// A high-level build/run target.
///
/// This type is intentionally conservative: it describes *what* the user wants to build for,
/// not how to do it. The suite-level builder uses this information to derive a plan.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Target {
    /// Architecture.
    pub arch: Arch,

    /// Whether the intended environment is bare metal.
    pub bare_metal: bool,

    /// Whether the intended firmware interface is UEFI.
    pub uefi: bool,
}

impl Target {
    /// Returns a canonical x86_64 bare-metal target.
    #[must_use]
    pub const fn x86_64_bare_metal() -> Self {
        Self {
            arch: Arch::X86_64,
            bare_metal: true,
            uefi: false,
        }
    }

    /// Returns a canonical x86_64 UEFI target.
    #[must_use]
    pub const fn x86_64_uefi() -> Self {
        Self {
            arch: Arch::X86_64,
            bare_metal: true,
            uefi: true,
        }
    }

    /// Returns a canonical AArch64 bare-metal target.
    #[must_use]
    pub const fn aarch64_bare_metal() -> Self {
        Self {
            arch: Arch::Aarch64,
            bare_metal: true,
            uefi: false,
        }
    }
}
