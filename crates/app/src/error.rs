use std::{fmt, panic::Location};

pub type AppResult<T> = Result<T, AppError>;
type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub trait ResultExt<T> {
    fn app_err(self, kind: AppErrorKind) -> AppResult<T>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    #[track_caller]
    fn app_err(self, kind: AppErrorKind) -> AppResult<T> {
        self.map_err(|err| AppError::from_source(kind, err))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AppErrorKind {
    Config,
    Bind,
    Serve,
}

impl fmt::Display for AppErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Config => "failed to load application configuration",
            Self::Bind => "failed to bind TCP listener",
            Self::Serve => "HTTP server terminated unexpectedly",
        };

        write!(f, "{message}")
    }
}

#[derive(Debug)]
pub struct AppError {
    kind: AppErrorKind,
    source: Option<BoxError>,
    location: &'static Location<'static>,
}

impl AppError {
    #[track_caller]
    pub fn from_source<E>(kind: AppErrorKind, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self {
            kind,
            source: Some(Box::new(source)),
            location: Location::caller(),
        }
    }

    pub fn log_debug(&self) {
        tracing::debug!(
            error_kind = %self.kind,
            error_file = self.file(),
            error_line = self.line(),
            error_column = self.column(),
            error = %self,
            source = ?self.source(),
            "application error"
        );
    }

    fn file(&self) -> &'static str {
        self.location.file()
    }

    fn line(&self) -> u32 {
        self.location.line()
    }

    fn column(&self) -> u32 {
        self.location.column()
    }

    pub fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn std::error::Error + 'static))
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source()
    }
}
