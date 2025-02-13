// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `BatchCreateRumMetricDefinitions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_create_rum_metric_definitions`](crate::client::Client::batch_create_rum_metric_definitions).
///
/// See [`crate::client::fluent_builders::BatchCreateRumMetricDefinitions`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchCreateRumMetricDefinitions {
    _private: (),
}
impl BatchCreateRumMetricDefinitions {
    /// Creates a new builder-style object to manufacture [`BatchCreateRumMetricDefinitionsInput`](crate::input::BatchCreateRumMetricDefinitionsInput).
    pub fn builder() -> crate::input::batch_create_rum_metric_definitions_input::Builder {
        crate::input::batch_create_rum_metric_definitions_input::Builder::default()
    }
    /// Creates a new `BatchCreateRumMetricDefinitions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchCreateRumMetricDefinitions {
    type Output = std::result::Result<
        crate::output::BatchCreateRumMetricDefinitionsOutput,
        crate::error::BatchCreateRumMetricDefinitionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_create_rum_metric_definitions_error(response)
        } else {
            crate::operation_deser::parse_batch_create_rum_metric_definitions_response(response)
        }
    }
}

/// Operation shape for `BatchDeleteRumMetricDefinitions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_delete_rum_metric_definitions`](crate::client::Client::batch_delete_rum_metric_definitions).
///
/// See [`crate::client::fluent_builders::BatchDeleteRumMetricDefinitions`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchDeleteRumMetricDefinitions {
    _private: (),
}
impl BatchDeleteRumMetricDefinitions {
    /// Creates a new builder-style object to manufacture [`BatchDeleteRumMetricDefinitionsInput`](crate::input::BatchDeleteRumMetricDefinitionsInput).
    pub fn builder() -> crate::input::batch_delete_rum_metric_definitions_input::Builder {
        crate::input::batch_delete_rum_metric_definitions_input::Builder::default()
    }
    /// Creates a new `BatchDeleteRumMetricDefinitions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchDeleteRumMetricDefinitions {
    type Output = std::result::Result<
        crate::output::BatchDeleteRumMetricDefinitionsOutput,
        crate::error::BatchDeleteRumMetricDefinitionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_delete_rum_metric_definitions_error(response)
        } else {
            crate::operation_deser::parse_batch_delete_rum_metric_definitions_response(response)
        }
    }
}

/// Operation shape for `BatchGetRumMetricDefinitions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_get_rum_metric_definitions`](crate::client::Client::batch_get_rum_metric_definitions).
///
/// See [`crate::client::fluent_builders::BatchGetRumMetricDefinitions`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchGetRumMetricDefinitions {
    _private: (),
}
impl BatchGetRumMetricDefinitions {
    /// Creates a new builder-style object to manufacture [`BatchGetRumMetricDefinitionsInput`](crate::input::BatchGetRumMetricDefinitionsInput).
    pub fn builder() -> crate::input::batch_get_rum_metric_definitions_input::Builder {
        crate::input::batch_get_rum_metric_definitions_input::Builder::default()
    }
    /// Creates a new `BatchGetRumMetricDefinitions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchGetRumMetricDefinitions {
    type Output = std::result::Result<
        crate::output::BatchGetRumMetricDefinitionsOutput,
        crate::error::BatchGetRumMetricDefinitionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_get_rum_metric_definitions_error(response)
        } else {
            crate::operation_deser::parse_batch_get_rum_metric_definitions_response(response)
        }
    }
}

/// Operation shape for `CreateAppMonitor`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_app_monitor`](crate::client::Client::create_app_monitor).
///
/// See [`crate::client::fluent_builders::CreateAppMonitor`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateAppMonitor {
    _private: (),
}
impl CreateAppMonitor {
    /// Creates a new builder-style object to manufacture [`CreateAppMonitorInput`](crate::input::CreateAppMonitorInput).
    pub fn builder() -> crate::input::create_app_monitor_input::Builder {
        crate::input::create_app_monitor_input::Builder::default()
    }
    /// Creates a new `CreateAppMonitor` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateAppMonitor {
    type Output = std::result::Result<
        crate::output::CreateAppMonitorOutput,
        crate::error::CreateAppMonitorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_app_monitor_error(response)
        } else {
            crate::operation_deser::parse_create_app_monitor_response(response)
        }
    }
}

/// Operation shape for `DeleteAppMonitor`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_app_monitor`](crate::client::Client::delete_app_monitor).
///
/// See [`crate::client::fluent_builders::DeleteAppMonitor`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteAppMonitor {
    _private: (),
}
impl DeleteAppMonitor {
    /// Creates a new builder-style object to manufacture [`DeleteAppMonitorInput`](crate::input::DeleteAppMonitorInput).
    pub fn builder() -> crate::input::delete_app_monitor_input::Builder {
        crate::input::delete_app_monitor_input::Builder::default()
    }
    /// Creates a new `DeleteAppMonitor` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteAppMonitor {
    type Output = std::result::Result<
        crate::output::DeleteAppMonitorOutput,
        crate::error::DeleteAppMonitorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_app_monitor_error(response)
        } else {
            crate::operation_deser::parse_delete_app_monitor_response(response)
        }
    }
}

/// Operation shape for `DeleteRumMetricsDestination`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_rum_metrics_destination`](crate::client::Client::delete_rum_metrics_destination).
///
/// See [`crate::client::fluent_builders::DeleteRumMetricsDestination`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteRumMetricsDestination {
    _private: (),
}
impl DeleteRumMetricsDestination {
    /// Creates a new builder-style object to manufacture [`DeleteRumMetricsDestinationInput`](crate::input::DeleteRumMetricsDestinationInput).
    pub fn builder() -> crate::input::delete_rum_metrics_destination_input::Builder {
        crate::input::delete_rum_metrics_destination_input::Builder::default()
    }
    /// Creates a new `DeleteRumMetricsDestination` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteRumMetricsDestination {
    type Output = std::result::Result<
        crate::output::DeleteRumMetricsDestinationOutput,
        crate::error::DeleteRumMetricsDestinationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_rum_metrics_destination_error(response)
        } else {
            crate::operation_deser::parse_delete_rum_metrics_destination_response(response)
        }
    }
}

/// Operation shape for `GetAppMonitor`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_app_monitor`](crate::client::Client::get_app_monitor).
///
/// See [`crate::client::fluent_builders::GetAppMonitor`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetAppMonitor {
    _private: (),
}
impl GetAppMonitor {
    /// Creates a new builder-style object to manufacture [`GetAppMonitorInput`](crate::input::GetAppMonitorInput).
    pub fn builder() -> crate::input::get_app_monitor_input::Builder {
        crate::input::get_app_monitor_input::Builder::default()
    }
    /// Creates a new `GetAppMonitor` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAppMonitor {
    type Output =
        std::result::Result<crate::output::GetAppMonitorOutput, crate::error::GetAppMonitorError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_app_monitor_error(response)
        } else {
            crate::operation_deser::parse_get_app_monitor_response(response)
        }
    }
}

/// Operation shape for `GetAppMonitorData`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_app_monitor_data`](crate::client::Client::get_app_monitor_data).
///
/// See [`crate::client::fluent_builders::GetAppMonitorData`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetAppMonitorData {
    _private: (),
}
impl GetAppMonitorData {
    /// Creates a new builder-style object to manufacture [`GetAppMonitorDataInput`](crate::input::GetAppMonitorDataInput).
    pub fn builder() -> crate::input::get_app_monitor_data_input::Builder {
        crate::input::get_app_monitor_data_input::Builder::default()
    }
    /// Creates a new `GetAppMonitorData` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAppMonitorData {
    type Output = std::result::Result<
        crate::output::GetAppMonitorDataOutput,
        crate::error::GetAppMonitorDataError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_app_monitor_data_error(response)
        } else {
            crate::operation_deser::parse_get_app_monitor_data_response(response)
        }
    }
}

/// Operation shape for `ListAppMonitors`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_app_monitors`](crate::client::Client::list_app_monitors).
///
/// See [`crate::client::fluent_builders::ListAppMonitors`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListAppMonitors {
    _private: (),
}
impl ListAppMonitors {
    /// Creates a new builder-style object to manufacture [`ListAppMonitorsInput`](crate::input::ListAppMonitorsInput).
    pub fn builder() -> crate::input::list_app_monitors_input::Builder {
        crate::input::list_app_monitors_input::Builder::default()
    }
    /// Creates a new `ListAppMonitors` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAppMonitors {
    type Output = std::result::Result<
        crate::output::ListAppMonitorsOutput,
        crate::error::ListAppMonitorsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_app_monitors_error(response)
        } else {
            crate::operation_deser::parse_list_app_monitors_response(response)
        }
    }
}

/// Operation shape for `ListRumMetricsDestinations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_rum_metrics_destinations`](crate::client::Client::list_rum_metrics_destinations).
///
/// See [`crate::client::fluent_builders::ListRumMetricsDestinations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListRumMetricsDestinations {
    _private: (),
}
impl ListRumMetricsDestinations {
    /// Creates a new builder-style object to manufacture [`ListRumMetricsDestinationsInput`](crate::input::ListRumMetricsDestinationsInput).
    pub fn builder() -> crate::input::list_rum_metrics_destinations_input::Builder {
        crate::input::list_rum_metrics_destinations_input::Builder::default()
    }
    /// Creates a new `ListRumMetricsDestinations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRumMetricsDestinations {
    type Output = std::result::Result<
        crate::output::ListRumMetricsDestinationsOutput,
        crate::error::ListRumMetricsDestinationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_rum_metrics_destinations_error(response)
        } else {
            crate::operation_deser::parse_list_rum_metrics_destinations_response(response)
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

/// Operation shape for `PutRumEvents`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_rum_events`](crate::client::Client::put_rum_events).
///
/// See [`crate::client::fluent_builders::PutRumEvents`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutRumEvents {
    _private: (),
}
impl PutRumEvents {
    /// Creates a new builder-style object to manufacture [`PutRumEventsInput`](crate::input::PutRumEventsInput).
    pub fn builder() -> crate::input::put_rum_events_input::Builder {
        crate::input::put_rum_events_input::Builder::default()
    }
    /// Creates a new `PutRumEvents` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutRumEvents {
    type Output =
        std::result::Result<crate::output::PutRumEventsOutput, crate::error::PutRumEventsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_rum_events_error(response)
        } else {
            crate::operation_deser::parse_put_rum_events_response(response)
        }
    }
}

/// Operation shape for `PutRumMetricsDestination`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_rum_metrics_destination`](crate::client::Client::put_rum_metrics_destination).
///
/// See [`crate::client::fluent_builders::PutRumMetricsDestination`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutRumMetricsDestination {
    _private: (),
}
impl PutRumMetricsDestination {
    /// Creates a new builder-style object to manufacture [`PutRumMetricsDestinationInput`](crate::input::PutRumMetricsDestinationInput).
    pub fn builder() -> crate::input::put_rum_metrics_destination_input::Builder {
        crate::input::put_rum_metrics_destination_input::Builder::default()
    }
    /// Creates a new `PutRumMetricsDestination` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutRumMetricsDestination {
    type Output = std::result::Result<
        crate::output::PutRumMetricsDestinationOutput,
        crate::error::PutRumMetricsDestinationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_rum_metrics_destination_error(response)
        } else {
            crate::operation_deser::parse_put_rum_metrics_destination_response(response)
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

/// Operation shape for `UpdateAppMonitor`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_app_monitor`](crate::client::Client::update_app_monitor).
///
/// See [`crate::client::fluent_builders::UpdateAppMonitor`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateAppMonitor {
    _private: (),
}
impl UpdateAppMonitor {
    /// Creates a new builder-style object to manufacture [`UpdateAppMonitorInput`](crate::input::UpdateAppMonitorInput).
    pub fn builder() -> crate::input::update_app_monitor_input::Builder {
        crate::input::update_app_monitor_input::Builder::default()
    }
    /// Creates a new `UpdateAppMonitor` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateAppMonitor {
    type Output = std::result::Result<
        crate::output::UpdateAppMonitorOutput,
        crate::error::UpdateAppMonitorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_app_monitor_error(response)
        } else {
            crate::operation_deser::parse_update_app_monitor_response(response)
        }
    }
}

/// Operation shape for `UpdateRumMetricDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_rum_metric_definition`](crate::client::Client::update_rum_metric_definition).
///
/// See [`crate::client::fluent_builders::UpdateRumMetricDefinition`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateRumMetricDefinition {
    _private: (),
}
impl UpdateRumMetricDefinition {
    /// Creates a new builder-style object to manufacture [`UpdateRumMetricDefinitionInput`](crate::input::UpdateRumMetricDefinitionInput).
    pub fn builder() -> crate::input::update_rum_metric_definition_input::Builder {
        crate::input::update_rum_metric_definition_input::Builder::default()
    }
    /// Creates a new `UpdateRumMetricDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateRumMetricDefinition {
    type Output = std::result::Result<
        crate::output::UpdateRumMetricDefinitionOutput,
        crate::error::UpdateRumMetricDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_rum_metric_definition_error(response)
        } else {
            crate::operation_deser::parse_update_rum_metric_definition_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
