// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct SendCommandError {
    pub kind: SendCommandErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum SendCommandErrorKind {
    BadRequestError(crate::error::BadRequestError),
    CapacityExceededError(crate::error::CapacityExceededError),
    InvalidSessionError(crate::error::InvalidSessionError),
    LimitExceededError(crate::error::LimitExceededError),
    OccConflictError(crate::error::OccConflictError),
    RateExceededError(crate::error::RateExceededError),

    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for SendCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            SendCommandErrorKind::BadRequestError(_inner) => _inner.fmt(f),
            SendCommandErrorKind::CapacityExceededError(_inner) => _inner.fmt(f),
            SendCommandErrorKind::InvalidSessionError(_inner) => _inner.fmt(f),
            SendCommandErrorKind::LimitExceededError(_inner) => _inner.fmt(f),
            SendCommandErrorKind::OccConflictError(_inner) => _inner.fmt(f),
            SendCommandErrorKind::RateExceededError(_inner) => _inner.fmt(f),
            SendCommandErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for SendCommandError {
    fn code(&self) -> Option<&str> {
        SendCommandError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}

impl SendCommandError {
    pub fn new(kind: SendCommandErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: SendCommandErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: SendCommandErrorKind::Unhandled(err.into()),
        }
    }

    // Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display as implemented
    // by std::Error to generate a message in that case.
    pub fn message(&self) -> Option<&str> {
        self.meta.message.as_deref()
    }

    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id.as_deref()
    }

    pub fn code(&self) -> Option<&str> {
        self.meta.code.as_deref()
    }
}

impl std::error::Error for SendCommandError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            SendCommandErrorKind::BadRequestError(_inner) => Some(_inner),
            SendCommandErrorKind::CapacityExceededError(_inner) => Some(_inner),
            SendCommandErrorKind::InvalidSessionError(_inner) => Some(_inner),
            SendCommandErrorKind::LimitExceededError(_inner) => Some(_inner),
            SendCommandErrorKind::OccConflictError(_inner) => Some(_inner),
            SendCommandErrorKind::RateExceededError(_inner) => Some(_inner),
            SendCommandErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>Returned when the rate of requests exceeds the allowed throughput.</p>
#[non_exhaustive]
#[derive(serde::Deserialize, serde::Serialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct RateExceededError {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for RateExceededError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RateExceededError");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl RateExceededError {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for RateExceededError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RateExceededError [RateExceededException]")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for RateExceededError {}
/// See [`RateExceededError`](crate::error::RateExceededError)
pub mod rate_exceeded_error {
    /// A builder for [`RateExceededError`](crate::error::RateExceededError)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, inp: impl Into<std::string::String>) -> Self {
            self.message = Some(inp.into());
            self
        }
        pub fn set_message(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.message = inp;
            self
        }
        /// Consumes the builder and constructs a [`RateExceededError`](crate::error::RateExceededError)
        pub fn build(self) -> crate::error::RateExceededError {
            crate::error::RateExceededError {
                message: self.message,
            }
        }
    }
}
impl RateExceededError {
    /// Creates a new builder-style object to manufacture [`RateExceededError`](crate::error::RateExceededError)
    pub fn builder() -> crate::error::rate_exceeded_error::Builder {
        crate::error::rate_exceeded_error::Builder::default()
    }
}

/// <p>Returned when a transaction cannot be written to the journal due to a failure in the
/// verification phase of <i>optimistic concurrency control</i> (OCC).</p>
#[non_exhaustive]
#[derive(serde::Deserialize, serde::Serialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct OccConflictError {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for OccConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("OccConflictError");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl OccConflictError {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for OccConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OccConflictError [OccConflictException]")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for OccConflictError {}
/// See [`OccConflictError`](crate::error::OccConflictError)
pub mod occ_conflict_error {
    /// A builder for [`OccConflictError`](crate::error::OccConflictError)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, inp: impl Into<std::string::String>) -> Self {
            self.message = Some(inp.into());
            self
        }
        pub fn set_message(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.message = inp;
            self
        }
        /// Consumes the builder and constructs a [`OccConflictError`](crate::error::OccConflictError)
        pub fn build(self) -> crate::error::OccConflictError {
            crate::error::OccConflictError {
                message: self.message,
            }
        }
    }
}
impl OccConflictError {
    /// Creates a new builder-style object to manufacture [`OccConflictError`](crate::error::OccConflictError)
    pub fn builder() -> crate::error::occ_conflict_error::Builder {
        crate::error::occ_conflict_error::Builder::default()
    }
}

/// <p>Returned if a resource limit such as number of active sessions is exceeded.</p>
#[non_exhaustive]
#[derive(serde::Deserialize, serde::Serialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct LimitExceededError {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for LimitExceededError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("LimitExceededError");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl LimitExceededError {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for LimitExceededError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LimitExceededError [LimitExceededException]")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for LimitExceededError {}
/// See [`LimitExceededError`](crate::error::LimitExceededError)
pub mod limit_exceeded_error {
    /// A builder for [`LimitExceededError`](crate::error::LimitExceededError)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, inp: impl Into<std::string::String>) -> Self {
            self.message = Some(inp.into());
            self
        }
        pub fn set_message(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.message = inp;
            self
        }
        /// Consumes the builder and constructs a [`LimitExceededError`](crate::error::LimitExceededError)
        pub fn build(self) -> crate::error::LimitExceededError {
            crate::error::LimitExceededError {
                message: self.message,
            }
        }
    }
}
impl LimitExceededError {
    /// Creates a new builder-style object to manufacture [`LimitExceededError`](crate::error::LimitExceededError)
    pub fn builder() -> crate::error::limit_exceeded_error::Builder {
        crate::error::limit_exceeded_error::Builder::default()
    }
}

/// <p>Returned if the session doesn't exist anymore because it timed out or expired.</p>
#[non_exhaustive]
#[derive(serde::Deserialize, serde::Serialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidSessionError {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub message: std::option::Option<std::string::String>,
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub code: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InvalidSessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidSessionError");
        formatter.field("message", &self.message);
        formatter.field("code", &self.code);
        formatter.finish()
    }
}
impl InvalidSessionError {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidSessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidSessionError [InvalidSessionException]")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidSessionError {}
/// See [`InvalidSessionError`](crate::error::InvalidSessionError)
pub mod invalid_session_error {
    /// A builder for [`InvalidSessionError`](crate::error::InvalidSessionError)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        pub(crate) code: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, inp: impl Into<std::string::String>) -> Self {
            self.message = Some(inp.into());
            self
        }
        pub fn set_message(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.message = inp;
            self
        }
        pub fn code(mut self, inp: impl Into<std::string::String>) -> Self {
            self.code = Some(inp.into());
            self
        }
        pub fn set_code(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.code = inp;
            self
        }
        /// Consumes the builder and constructs a [`InvalidSessionError`](crate::error::InvalidSessionError)
        pub fn build(self) -> crate::error::InvalidSessionError {
            crate::error::InvalidSessionError {
                message: self.message,
                code: self.code,
            }
        }
    }
}
impl InvalidSessionError {
    /// Creates a new builder-style object to manufacture [`InvalidSessionError`](crate::error::InvalidSessionError)
    pub fn builder() -> crate::error::invalid_session_error::Builder {
        crate::error::invalid_session_error::Builder::default()
    }
}

/// <p>Returned when the request exceeds the processing capacity of the ledger.</p>
#[non_exhaustive]
#[derive(serde::Deserialize, serde::Serialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct CapacityExceededError {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for CapacityExceededError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CapacityExceededError");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl CapacityExceededError {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for CapacityExceededError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CapacityExceededError [CapacityExceededException]")?;
        if let Some(inner_5) = &self.message {
            write!(f, ": {}", inner_5)?;
        }
        Ok(())
    }
}
impl std::error::Error for CapacityExceededError {}
/// See [`CapacityExceededError`](crate::error::CapacityExceededError)
pub mod capacity_exceeded_error {
    /// A builder for [`CapacityExceededError`](crate::error::CapacityExceededError)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, inp: impl Into<std::string::String>) -> Self {
            self.message = Some(inp.into());
            self
        }
        pub fn set_message(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.message = inp;
            self
        }
        /// Consumes the builder and constructs a [`CapacityExceededError`](crate::error::CapacityExceededError)
        pub fn build(self) -> crate::error::CapacityExceededError {
            crate::error::CapacityExceededError {
                message: self.message,
            }
        }
    }
}
impl CapacityExceededError {
    /// Creates a new builder-style object to manufacture [`CapacityExceededError`](crate::error::CapacityExceededError)
    pub fn builder() -> crate::error::capacity_exceeded_error::Builder {
        crate::error::capacity_exceeded_error::Builder::default()
    }
}

/// <p>Returned if the request is malformed or contains an error such as an invalid parameter
/// value or a missing required parameter.</p>
#[non_exhaustive]
#[derive(serde::Deserialize, serde::Serialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct BadRequestError {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub message: std::option::Option<std::string::String>,
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub code: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for BadRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("BadRequestError");
        formatter.field("message", &self.message);
        formatter.field("code", &self.code);
        formatter.finish()
    }
}
impl BadRequestError {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for BadRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BadRequestError [BadRequestException]")?;
        if let Some(inner_6) = &self.message {
            write!(f, ": {}", inner_6)?;
        }
        Ok(())
    }
}
impl std::error::Error for BadRequestError {}
/// See [`BadRequestError`](crate::error::BadRequestError)
pub mod bad_request_error {
    /// A builder for [`BadRequestError`](crate::error::BadRequestError)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        pub(crate) code: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, inp: impl Into<std::string::String>) -> Self {
            self.message = Some(inp.into());
            self
        }
        pub fn set_message(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.message = inp;
            self
        }
        pub fn code(mut self, inp: impl Into<std::string::String>) -> Self {
            self.code = Some(inp.into());
            self
        }
        pub fn set_code(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.code = inp;
            self
        }
        /// Consumes the builder and constructs a [`BadRequestError`](crate::error::BadRequestError)
        pub fn build(self) -> crate::error::BadRequestError {
            crate::error::BadRequestError {
                message: self.message,
                code: self.code,
            }
        }
    }
}
impl BadRequestError {
    /// Creates a new builder-style object to manufacture [`BadRequestError`](crate::error::BadRequestError)
    pub fn builder() -> crate::error::bad_request_error::Builder {
        crate::error::bad_request_error::Builder::default()
    }
}
