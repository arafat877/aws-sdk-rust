// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>There is an error in the call or in a SQL statement.</p>
    BadRequestException(crate::error::BadRequestException),
    /// <p>There are insufficient privileges to make the call.</p>
    ForbiddenException(crate::error::ForbiddenException),
    /// <p>An internal error occurred.</p>
    InternalServerErrorException(crate::error::InternalServerErrorException),
    /// <p>The <code>resourceArn</code>, <code>secretArn</code>, or <code>transactionId</code> value can't be found.</p>
    NotFoundException(crate::error::NotFoundException),
    /// <p>The service specified by the <code>resourceArn</code> parameter is not available.</p>
    ServiceUnavailableError(crate::error::ServiceUnavailableError),
    /// <p>The execution of the SQL statement timed out.</p>
    StatementTimeoutException(crate::error::StatementTimeoutException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::BadRequestException(inner) => inner.fmt(f),
            Error::ForbiddenException(inner) => inner.fmt(f),
            Error::InternalServerErrorException(inner) => inner.fmt(f),
            Error::NotFoundException(inner) => inner.fmt(f),
            Error::ServiceUnavailableError(inner) => inner.fmt(f),
            Error::StatementTimeoutException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::BatchExecuteStatementError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::BatchExecuteStatementError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::BatchExecuteStatementError> for Error {
    fn from(err: crate::error::BatchExecuteStatementError) -> Self {
        match err.kind {
            crate::error::BatchExecuteStatementErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::BatchExecuteStatementErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::BatchExecuteStatementErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::BatchExecuteStatementErrorKind::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::error::BatchExecuteStatementErrorKind::ServiceUnavailableError(inner) => {
                Error::ServiceUnavailableError(inner)
            }
            crate::error::BatchExecuteStatementErrorKind::StatementTimeoutException(inner) => {
                Error::StatementTimeoutException(inner)
            }
            crate::error::BatchExecuteStatementErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::BeginTransactionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::BeginTransactionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::BeginTransactionError> for Error {
    fn from(err: crate::error::BeginTransactionError) -> Self {
        match err.kind {
            crate::error::BeginTransactionErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::BeginTransactionErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::BeginTransactionErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::BeginTransactionErrorKind::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::error::BeginTransactionErrorKind::ServiceUnavailableError(inner) => {
                Error::ServiceUnavailableError(inner)
            }
            crate::error::BeginTransactionErrorKind::StatementTimeoutException(inner) => {
                Error::StatementTimeoutException(inner)
            }
            crate::error::BeginTransactionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CommitTransactionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CommitTransactionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CommitTransactionError> for Error {
    fn from(err: crate::error::CommitTransactionError) -> Self {
        match err.kind {
            crate::error::CommitTransactionErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::CommitTransactionErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::CommitTransactionErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::CommitTransactionErrorKind::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::error::CommitTransactionErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::CommitTransactionErrorKind::ServiceUnavailableError(inner) => {
                Error::ServiceUnavailableError(inner)
            }
            crate::error::CommitTransactionErrorKind::StatementTimeoutException(inner) => {
                Error::StatementTimeoutException(inner)
            }
            crate::error::CommitTransactionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ExecuteSqlError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ExecuteSqlError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ExecuteSqlError> for Error {
    fn from(err: crate::error::ExecuteSqlError) -> Self {
        match err.kind {
            crate::error::ExecuteSqlErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ExecuteSqlErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::ExecuteSqlErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::ExecuteSqlErrorKind::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::error::ExecuteSqlErrorKind::ServiceUnavailableError(inner) => {
                Error::ServiceUnavailableError(inner)
            }
            crate::error::ExecuteSqlErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ExecuteStatementError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ExecuteStatementError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ExecuteStatementError> for Error {
    fn from(err: crate::error::ExecuteStatementError) -> Self {
        match err.kind {
            crate::error::ExecuteStatementErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ExecuteStatementErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::ExecuteStatementErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::ExecuteStatementErrorKind::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::error::ExecuteStatementErrorKind::ServiceUnavailableError(inner) => {
                Error::ServiceUnavailableError(inner)
            }
            crate::error::ExecuteStatementErrorKind::StatementTimeoutException(inner) => {
                Error::StatementTimeoutException(inner)
            }
            crate::error::ExecuteStatementErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RollbackTransactionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RollbackTransactionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::RollbackTransactionError> for Error {
    fn from(err: crate::error::RollbackTransactionError) -> Self {
        match err.kind {
            crate::error::RollbackTransactionErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::RollbackTransactionErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::RollbackTransactionErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::RollbackTransactionErrorKind::InternalServerErrorException(inner) => {
                Error::InternalServerErrorException(inner)
            }
            crate::error::RollbackTransactionErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::RollbackTransactionErrorKind::ServiceUnavailableError(inner) => {
                Error::ServiceUnavailableError(inner)
            }
            crate::error::RollbackTransactionErrorKind::StatementTimeoutException(inner) => {
                Error::StatementTimeoutException(inner)
            }
            crate::error::RollbackTransactionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
