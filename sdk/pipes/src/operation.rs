// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreatePipe`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_pipe`](crate::client::Client::create_pipe).
///
/// See [`crate::client::fluent_builders::CreatePipe`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreatePipe {
    _private: (),
}
impl CreatePipe {
    /// Creates a new builder-style object to manufacture [`CreatePipeInput`](crate::input::CreatePipeInput).
    pub fn builder() -> crate::input::create_pipe_input::Builder {
        crate::input::create_pipe_input::Builder::default()
    }
    /// Creates a new `CreatePipe` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreatePipe {
    type Output =
        std::result::Result<crate::output::CreatePipeOutput, crate::error::CreatePipeError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_pipe_error(response)
        } else {
            crate::operation_deser::parse_create_pipe_response(response)
        }
    }
}

/// Operation shape for `DeletePipe`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_pipe`](crate::client::Client::delete_pipe).
///
/// See [`crate::client::fluent_builders::DeletePipe`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeletePipe {
    _private: (),
}
impl DeletePipe {
    /// Creates a new builder-style object to manufacture [`DeletePipeInput`](crate::input::DeletePipeInput).
    pub fn builder() -> crate::input::delete_pipe_input::Builder {
        crate::input::delete_pipe_input::Builder::default()
    }
    /// Creates a new `DeletePipe` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeletePipe {
    type Output =
        std::result::Result<crate::output::DeletePipeOutput, crate::error::DeletePipeError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_pipe_error(response)
        } else {
            crate::operation_deser::parse_delete_pipe_response(response)
        }
    }
}

/// Operation shape for `DescribePipe`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_pipe`](crate::client::Client::describe_pipe).
///
/// See [`crate::client::fluent_builders::DescribePipe`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribePipe {
    _private: (),
}
impl DescribePipe {
    /// Creates a new builder-style object to manufacture [`DescribePipeInput`](crate::input::DescribePipeInput).
    pub fn builder() -> crate::input::describe_pipe_input::Builder {
        crate::input::describe_pipe_input::Builder::default()
    }
    /// Creates a new `DescribePipe` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribePipe {
    type Output =
        std::result::Result<crate::output::DescribePipeOutput, crate::error::DescribePipeError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_pipe_error(response)
        } else {
            crate::operation_deser::parse_describe_pipe_response(response)
        }
    }
}

/// Operation shape for `ListPipes`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_pipes`](crate::client::Client::list_pipes).
///
/// See [`crate::client::fluent_builders::ListPipes`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListPipes {
    _private: (),
}
impl ListPipes {
    /// Creates a new builder-style object to manufacture [`ListPipesInput`](crate::input::ListPipesInput).
    pub fn builder() -> crate::input::list_pipes_input::Builder {
        crate::input::list_pipes_input::Builder::default()
    }
    /// Creates a new `ListPipes` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPipes {
    type Output = std::result::Result<crate::output::ListPipesOutput, crate::error::ListPipesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_pipes_error(response)
        } else {
            crate::operation_deser::parse_list_pipes_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `StartPipe`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_pipe`](crate::client::Client::start_pipe).
///
/// See [`crate::client::fluent_builders::StartPipe`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartPipe {
    _private: (),
}
impl StartPipe {
    /// Creates a new builder-style object to manufacture [`StartPipeInput`](crate::input::StartPipeInput).
    pub fn builder() -> crate::input::start_pipe_input::Builder {
        crate::input::start_pipe_input::Builder::default()
    }
    /// Creates a new `StartPipe` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartPipe {
    type Output = std::result::Result<crate::output::StartPipeOutput, crate::error::StartPipeError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_pipe_error(response)
        } else {
            crate::operation_deser::parse_start_pipe_response(response)
        }
    }
}

/// Operation shape for `StopPipe`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`stop_pipe`](crate::client::Client::stop_pipe).
///
/// See [`crate::client::fluent_builders::StopPipe`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StopPipe {
    _private: (),
}
impl StopPipe {
    /// Creates a new builder-style object to manufacture [`StopPipeInput`](crate::input::StopPipeInput).
    pub fn builder() -> crate::input::stop_pipe_input::Builder {
        crate::input::stop_pipe_input::Builder::default()
    }
    /// Creates a new `StopPipe` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopPipe {
    type Output = std::result::Result<crate::output::StopPipeOutput, crate::error::StopPipeError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_pipe_error(response)
        } else {
            crate::operation_deser::parse_stop_pipe_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdatePipe`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_pipe`](crate::client::Client::update_pipe).
///
/// See [`crate::client::fluent_builders::UpdatePipe`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdatePipe {
    _private: (),
}
impl UpdatePipe {
    /// Creates a new builder-style object to manufacture [`UpdatePipeInput`](crate::input::UpdatePipeInput).
    pub fn builder() -> crate::input::update_pipe_input::Builder {
        crate::input::update_pipe_input::Builder::default()
    }
    /// Creates a new `UpdatePipe` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdatePipe {
    type Output =
        std::result::Result<crate::output::UpdatePipeOutput, crate::error::UpdatePipeError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_pipe_error(response)
        } else {
            crate::operation_deser::parse_update_pipe_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
