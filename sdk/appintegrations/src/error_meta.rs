// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>A resource with the specified name already exists.</p>
    DuplicateResourceException(crate::error::DuplicateResourceException),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalServiceError(crate::error::InternalServiceError),
    /// <p>The request is not valid. </p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p>The specified resource was not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The allowed quota for the resource has been exceeded.</p>
    ResourceQuotaExceededException(crate::error::ResourceQuotaExceededException),
    /// <p>The throttling limit has been exceeded.</p>
    ThrottlingException(crate::error::ThrottlingException),
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
            Error::DuplicateResourceException(inner) => inner.fmt(f),
            Error::InternalServiceError(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ResourceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateDataIntegrationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateDataIntegrationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateDataIntegrationError> for Error {
    fn from(err: crate::error::CreateDataIntegrationError) -> Self {
        match err.kind {
            crate::error::CreateDataIntegrationErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::CreateDataIntegrationErrorKind::DuplicateResourceException(inner) => {
                Error::DuplicateResourceException(inner)
            }
            crate::error::CreateDataIntegrationErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::CreateDataIntegrationErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::CreateDataIntegrationErrorKind::ResourceQuotaExceededException(inner) => {
                Error::ResourceQuotaExceededException(inner)
            }
            crate::error::CreateDataIntegrationErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::CreateDataIntegrationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateEventIntegrationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateEventIntegrationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateEventIntegrationError> for Error {
    fn from(err: crate::error::CreateEventIntegrationError) -> Self {
        match err.kind {
            crate::error::CreateEventIntegrationErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::CreateEventIntegrationErrorKind::DuplicateResourceException(inner) => {
                Error::DuplicateResourceException(inner)
            }
            crate::error::CreateEventIntegrationErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::CreateEventIntegrationErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::CreateEventIntegrationErrorKind::ResourceQuotaExceededException(
                inner,
            ) => Error::ResourceQuotaExceededException(inner),
            crate::error::CreateEventIntegrationErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::CreateEventIntegrationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteDataIntegrationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteDataIntegrationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteDataIntegrationError> for Error {
    fn from(err: crate::error::DeleteDataIntegrationError) -> Self {
        match err.kind {
            crate::error::DeleteDataIntegrationErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::DeleteDataIntegrationErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::DeleteDataIntegrationErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::DeleteDataIntegrationErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DeleteDataIntegrationErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DeleteDataIntegrationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteEventIntegrationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteEventIntegrationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteEventIntegrationError> for Error {
    fn from(err: crate::error::DeleteEventIntegrationError) -> Self {
        match err.kind {
            crate::error::DeleteEventIntegrationErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::DeleteEventIntegrationErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::DeleteEventIntegrationErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::DeleteEventIntegrationErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DeleteEventIntegrationErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DeleteEventIntegrationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetDataIntegrationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetDataIntegrationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetDataIntegrationError> for Error {
    fn from(err: crate::error::GetDataIntegrationError) -> Self {
        match err.kind {
            crate::error::GetDataIntegrationErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::GetDataIntegrationErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::GetDataIntegrationErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::GetDataIntegrationErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetDataIntegrationErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::GetDataIntegrationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetEventIntegrationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetEventIntegrationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetEventIntegrationError> for Error {
    fn from(err: crate::error::GetEventIntegrationError) -> Self {
        match err.kind {
            crate::error::GetEventIntegrationErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::GetEventIntegrationErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::GetEventIntegrationErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::GetEventIntegrationErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetEventIntegrationErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::GetEventIntegrationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::ListDataIntegrationAssociationsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::ListDataIntegrationAssociationsError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListDataIntegrationAssociationsError> for Error {
    fn from(err: crate::error::ListDataIntegrationAssociationsError) -> Self {
        match err.kind {
            crate::error::ListDataIntegrationAssociationsErrorKind::AccessDeniedException(
                inner,
            ) => Error::AccessDeniedException(inner),
            crate::error::ListDataIntegrationAssociationsErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::ListDataIntegrationAssociationsErrorKind::InvalidRequestException(
                inner,
            ) => Error::InvalidRequestException(inner),
            crate::error::ListDataIntegrationAssociationsErrorKind::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::error::ListDataIntegrationAssociationsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListDataIntegrationAssociationsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListDataIntegrationsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListDataIntegrationsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListDataIntegrationsError> for Error {
    fn from(err: crate::error::ListDataIntegrationsError) -> Self {
        match err.kind {
            crate::error::ListDataIntegrationsErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListDataIntegrationsErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::ListDataIntegrationsErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ListDataIntegrationsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListDataIntegrationsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::ListEventIntegrationAssociationsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::ListEventIntegrationAssociationsError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListEventIntegrationAssociationsError> for Error {
    fn from(err: crate::error::ListEventIntegrationAssociationsError) -> Self {
        match err.kind {
            crate::error::ListEventIntegrationAssociationsErrorKind::AccessDeniedException(
                inner,
            ) => Error::AccessDeniedException(inner),
            crate::error::ListEventIntegrationAssociationsErrorKind::InternalServiceError(
                inner,
            ) => Error::InternalServiceError(inner),
            crate::error::ListEventIntegrationAssociationsErrorKind::InvalidRequestException(
                inner,
            ) => Error::InvalidRequestException(inner),
            crate::error::ListEventIntegrationAssociationsErrorKind::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::error::ListEventIntegrationAssociationsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListEventIntegrationAssociationsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListEventIntegrationsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListEventIntegrationsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListEventIntegrationsError> for Error {
    fn from(err: crate::error::ListEventIntegrationsError) -> Self {
        match err.kind {
            crate::error::ListEventIntegrationsErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListEventIntegrationsErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::ListEventIntegrationsErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ListEventIntegrationsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListEventIntegrationsErrorKind::Unhandled(inner) => {
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
            crate::error::ListTagsForResourceErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::ListTagsForResourceErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
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
            crate::error::TagResourceErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::TagResourceErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::TagResourceErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
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
            crate::error::UntagResourceErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::UntagResourceErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UntagResourceErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::UntagResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateDataIntegrationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateDataIntegrationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateDataIntegrationError> for Error {
    fn from(err: crate::error::UpdateDataIntegrationError) -> Self {
        match err.kind {
            crate::error::UpdateDataIntegrationErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::UpdateDataIntegrationErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::UpdateDataIntegrationErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::UpdateDataIntegrationErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UpdateDataIntegrationErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::UpdateDataIntegrationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateEventIntegrationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateEventIntegrationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateEventIntegrationError> for Error {
    fn from(err: crate::error::UpdateEventIntegrationError) -> Self {
        match err.kind {
            crate::error::UpdateEventIntegrationErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::UpdateEventIntegrationErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::UpdateEventIntegrationErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::UpdateEventIntegrationErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UpdateEventIntegrationErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::UpdateEventIntegrationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
