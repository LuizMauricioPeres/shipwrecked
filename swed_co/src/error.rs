//! Error types shared across the SWed workspace.

use std::fmt;

/// Diagnostic severity levels used across the SWed pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SeverityLevel {
    /// Informational — safe to ignore.
    Notice,
    /// Something unexpected, but transpilation continues.
    Warning,
    /// Transpilation produced incorrect output; runtime may fail.
    Critical,
    /// Unrecoverable — process must abort.
    Panic,
}

impl fmt::Display for SeverityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Notice   => write!(f, "notice"),
            Self::Warning  => write!(f, "warning"),
            Self::Critical => write!(f, "critical"),
            Self::Panic    => write!(f, "panic"),
        }
    }
}

/// Source location (byte offsets into the `.prg` source string).
#[derive(Debug, Clone, Default)]
pub struct SrcSpan {
    /// Byte offset of the first character.
    pub start: usize,
    /// Byte offset past the last character.
    pub end: usize,
}

impl SrcSpan {
    /// Construct a span from start..end byte offsets.
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// Typed error produced by any SWed crate.
#[derive(Debug, Clone)]
pub struct SwedError {
    /// Severity level.
    pub level: SeverityLevel,
    /// Human-readable description.
    pub message: String,
    /// Source location where the error was detected.
    pub span: SrcSpan,
}

impl SwedError {
    /// Convenience constructor.
    pub fn new(level: SeverityLevel, message: impl Into<String>, span: SrcSpan) -> Self {
        Self { level, message: message.into(), span }
    }

    /// Shorthand for a `Notice`.
    pub fn notice(message: impl Into<String>) -> Self {
        Self::new(SeverityLevel::Notice, message, SrcSpan::default())
    }

    /// Shorthand for a `Warning`.
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(SeverityLevel::Warning, message, SrcSpan::default())
    }

    /// Shorthand for a `Critical` error.
    pub fn critical(message: impl Into<String>) -> Self {
        Self::new(SeverityLevel::Critical, message, SrcSpan::default())
    }
}

impl fmt::Display for SwedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.level, self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_ordering() {
        assert!(SeverityLevel::Notice < SeverityLevel::Warning);
        assert!(SeverityLevel::Warning < SeverityLevel::Critical);
        assert!(SeverityLevel::Critical < SeverityLevel::Panic);
    }

    #[test]
    fn swed_error_display() {
        let e = SwedError::warning("undeclared variable");
        assert_eq!(format!("{e}"), "[warning] undeclared variable");
    }

    #[test]
    fn swed_error_critical() {
        let e = SwedError::critical("type mismatch");
        assert_eq!(e.level, SeverityLevel::Critical);
    }
}
