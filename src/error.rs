use core::fmt;

/// The suite-level result type.
pub type Result<T> = core::result::Result<T, Error>;

/// Suite-level error type.
///
/// This error is intentionally small and stable. It represents problems in suite-level
/// orchestration (configuration validation, unsupported targets, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The blueprint is invalid.
    InvalidBlueprint {
        /// A short, human-readable explanation of what failed validation.
        message: &'static str,
    },

    /// The requested target is not supported by the selected profile or environment.
    UnsupportedTarget {
        /// A short, human-readable explanation.
        message: &'static str,
    },
}

impl Error {
    /// Creates an [`Error::InvalidBlueprint`].
    #[must_use]
    pub const fn invalid_blueprint(message: &'static str) -> Self {
        Self::InvalidBlueprint { message }
    }

    /// Creates an [`Error::UnsupportedTarget`].
    #[must_use]
    pub const fn unsupported_target(message: &'static str) -> Self {
        Self::UnsupportedTarget { message }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBlueprint { message } => write!(f, "invalid blueprint: {message}"),
            Self::UnsupportedTarget { message } => write!(f, "unsupported target: {message}"),
        }
    }
}

impl std::error::Error for Error {}
