// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>A resource was in an inconsistent state during an update or a deletion.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>Unexpected error while processing the request. Retry the request.</p>
    InternalServiceFault(crate::error::InternalServiceFault),
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameterException(crate::error::InvalidParameterException),
    /// <p>A required parameter is missing from the request.</p>
    MissingRequiredParameterException(crate::error::MissingRequiredParameterException),
    /// <p>The request references a resource that does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The request would cause a service quota to be exceeded.</p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p>A resource can have no more than 50 tags.</p>
    TooManyTagsException(crate::error::TooManyTagsException),
    /// <p>The value of a parameter in the request caused an error.</p>
    ValidationException(crate::error::ValidationException),
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
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServiceFault(inner) => inner.fmt(f),
            Error::InvalidParameterException(inner) => inner.fmt(f),
            Error::MissingRequiredParameterException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::TooManyTagsException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateLinkError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateLinkError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateLinkError> for Error {
    fn from(err: crate::error::CreateLinkError) -> Self {
        match err.kind {
            crate::error::CreateLinkErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::CreateLinkErrorKind::InternalServiceFault(inner) => {
                Error::InternalServiceFault(inner)
            }
            crate::error::CreateLinkErrorKind::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::error::CreateLinkErrorKind::MissingRequiredParameterException(inner) => {
                Error::MissingRequiredParameterException(inner)
            }
            crate::error::CreateLinkErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::CreateLinkErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateSinkError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateSinkError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateSinkError> for Error {
    fn from(err: crate::error::CreateSinkError) -> Self {
        match err.kind {
            crate::error::CreateSinkErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::CreateSinkErrorKind::InternalServiceFault(inner) => {
                Error::InternalServiceFault(inner)
            }
            crate::error::CreateSinkErrorKind::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::error::CreateSinkErrorKind::MissingRequiredParameterException(inner) => {
                Error::MissingRequiredParameterException(inner)
            }
            crate::error::CreateSinkErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::CreateSinkErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteLinkError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteLinkError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteLinkError> for Error {
    fn from(err: crate::error::DeleteLinkError) -> Self {
        match err.kind {
            crate::error::DeleteLinkErrorKind::InternalServiceFault(inner) => {
                Error::InternalServiceFault(inner)
            }
            crate::error::DeleteLinkErrorKind::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::error::DeleteLinkErrorKind::MissingRequiredParameterException(inner) => {
                Error::MissingRequiredParameterException(inner)
            }
            crate::error::DeleteLinkErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DeleteLinkErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteSinkError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteSinkError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteSinkError> for Error {
    fn from(err: crate::error::DeleteSinkError) -> Self {
        match err.kind {
            crate::error::DeleteSinkErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::DeleteSinkErrorKind::InternalServiceFault(inner) => {
                Error::InternalServiceFault(inner)
            }
            crate::error::DeleteSinkErrorKind::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::error::DeleteSinkErrorKind::MissingRequiredParameterException(inner) => {
                Error::MissingRequiredParameterException(inner)
            }
            crate::error::DeleteSinkErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DeleteSinkErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetLinkError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetLinkError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetLinkError> for Error {
    fn from(err: crate::error::GetLinkError) -> Self {
        match err.kind {
            crate::error::GetLinkErrorKind::InternalServiceFault(inner) => {
                Error::InternalServiceFault(inner)
            }
            crate::error::GetLinkErrorKind::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::error::GetLinkErrorKind::MissingRequiredParameterException(inner) => {
                Error::MissingRequiredParameterException(inner)
            }
            crate::error::GetLinkErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetLinkErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSinkError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetSinkError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetSinkError> for Error {
    fn from(err: crate::error::GetSinkError) -> Self {
        match err.kind {
            crate::error::GetSinkErrorKind::InternalServiceFault(inner) => {
                Error::InternalServiceFault(inner)
            }
            crate::error::GetSinkErrorKind::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::error::GetSinkErrorKind::MissingRequiredParameterException(inner) => {
                Error::MissingRequiredParameterException(inner)
            }
            crate::error::GetSinkErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetSinkErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSinkPolicyError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetSinkPolicyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetSinkPolicyError> for Error {
    fn from(err: crate::error::GetSinkPolicyError) -> Self {
        match err.kind {
            crate::error::GetSinkPolicyErrorKind::InternalServiceFault(inner) => {
                Error::InternalServiceFault(inner)
            }
            crate::error::GetSinkPolicyErrorKind::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::error::GetSinkPolicyErrorKind::MissingRequiredParameterException(inner) => {
                Error::MissingRequiredParameterException(inner)
            }
            crate::error::GetSinkPolicyErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetSinkPolicyErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListAttachedLinksError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListAttachedLinksError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListAttachedLinksError> for Error {
    fn from(err: crate::error::ListAttachedLinksError) -> Self {
        match err.kind {
            crate::error::ListAttachedLinksErrorKind::InternalServiceFault(inner) => {
                Error::InternalServiceFault(inner)
            }
            crate::error::ListAttachedLinksErrorKind::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::error::ListAttachedLinksErrorKind::MissingRequiredParameterException(inner) => {
                Error::MissingRequiredParameterException(inner)
            }
            crate::error::ListAttachedLinksErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListAttachedLinksErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListLinksError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListLinksError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListLinksError> for Error {
    fn from(err: crate::error::ListLinksError) -> Self {
        match err.kind {
            crate::error::ListLinksErrorKind::InternalServiceFault(inner) => {
                Error::InternalServiceFault(inner)
            }
            crate::error::ListLinksErrorKind::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::error::ListLinksErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListLinksErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListSinksError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListSinksError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListSinksError> for Error {
    fn from(err: crate::error::ListSinksError) -> Self {
        match err.kind {
            crate::error::ListSinksErrorKind::InternalServiceFault(inner) => {
                Error::InternalServiceFault(inner)
            }
            crate::error::ListSinksErrorKind::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::error::ListSinksErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListSinksErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListTagsForResourceError> for Error {
    fn from(err: crate::error::ListTagsForResourceError) -> Self {
        match err.kind {
            crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutSinkPolicyError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutSinkPolicyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::PutSinkPolicyError> for Error {
    fn from(err: crate::error::PutSinkPolicyError) -> Self {
        match err.kind {
            crate::error::PutSinkPolicyErrorKind::InternalServiceFault(inner) => {
                Error::InternalServiceFault(inner)
            }
            crate::error::PutSinkPolicyErrorKind::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::error::PutSinkPolicyErrorKind::MissingRequiredParameterException(inner) => {
                Error::MissingRequiredParameterException(inner)
            }
            crate::error::PutSinkPolicyErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::PutSinkPolicyErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::TagResourceError> for Error {
    fn from(err: crate::error::TagResourceError) -> Self {
        match err.kind {
            crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::TagResourceErrorKind::TooManyTagsException(inner) => {
                Error::TooManyTagsException(inner)
            }
            crate::error::TagResourceErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::TagResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UntagResourceError> for Error {
    fn from(err: crate::error::UntagResourceError) -> Self {
        match err.kind {
            crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UntagResourceErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::UntagResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateLinkError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateLinkError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateLinkError> for Error {
    fn from(err: crate::error::UpdateLinkError) -> Self {
        match err.kind {
            crate::error::UpdateLinkErrorKind::InternalServiceFault(inner) => {
                Error::InternalServiceFault(inner)
            }
            crate::error::UpdateLinkErrorKind::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::error::UpdateLinkErrorKind::MissingRequiredParameterException(inner) => {
                Error::MissingRequiredParameterException(inner)
            }
            crate::error::UpdateLinkErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UpdateLinkErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
