// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateDestination`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_destination`](crate::client::Client::create_destination).
///
/// See [`crate::client::fluent_builders::CreateDestination`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateDestination {
    _private: (),
}
impl CreateDestination {
    /// Creates a new builder-style object to manufacture [`CreateDestinationInput`](crate::input::CreateDestinationInput).
    pub fn builder() -> crate::input::create_destination_input::Builder {
        crate::input::create_destination_input::Builder::default()
    }
    /// Creates a new `CreateDestination` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateDestination {
    type Output = std::result::Result<
        crate::output::CreateDestinationOutput,
        crate::error::CreateDestinationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_destination_error(response)
        } else {
            crate::operation_deser::parse_create_destination_response(response)
        }
    }
}

/// Operation shape for `CreateSite`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_site`](crate::client::Client::create_site).
///
/// See [`crate::client::fluent_builders::CreateSite`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateSite {
    _private: (),
}
impl CreateSite {
    /// Creates a new builder-style object to manufacture [`CreateSiteInput`](crate::input::CreateSiteInput).
    pub fn builder() -> crate::input::create_site_input::Builder {
        crate::input::create_site_input::Builder::default()
    }
    /// Creates a new `CreateSite` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateSite {
    type Output =
        std::result::Result<crate::output::CreateSiteOutput, crate::error::CreateSiteError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_site_error(response)
        } else {
            crate::operation_deser::parse_create_site_response(response)
        }
    }
}

/// Operation shape for `CreateWorker`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_worker`](crate::client::Client::create_worker).
///
/// See [`crate::client::fluent_builders::CreateWorker`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateWorker {
    _private: (),
}
impl CreateWorker {
    /// Creates a new builder-style object to manufacture [`CreateWorkerInput`](crate::input::CreateWorkerInput).
    pub fn builder() -> crate::input::create_worker_input::Builder {
        crate::input::create_worker_input::Builder::default()
    }
    /// Creates a new `CreateWorker` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateWorker {
    type Output =
        std::result::Result<crate::output::CreateWorkerOutput, crate::error::CreateWorkerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_worker_error(response)
        } else {
            crate::operation_deser::parse_create_worker_response(response)
        }
    }
}

/// Operation shape for `CreateWorkerFleet`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_worker_fleet`](crate::client::Client::create_worker_fleet).
///
/// See [`crate::client::fluent_builders::CreateWorkerFleet`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateWorkerFleet {
    _private: (),
}
impl CreateWorkerFleet {
    /// Creates a new builder-style object to manufacture [`CreateWorkerFleetInput`](crate::input::CreateWorkerFleetInput).
    pub fn builder() -> crate::input::create_worker_fleet_input::Builder {
        crate::input::create_worker_fleet_input::Builder::default()
    }
    /// Creates a new `CreateWorkerFleet` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateWorkerFleet {
    type Output = std::result::Result<
        crate::output::CreateWorkerFleetOutput,
        crate::error::CreateWorkerFleetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_worker_fleet_error(response)
        } else {
            crate::operation_deser::parse_create_worker_fleet_response(response)
        }
    }
}

/// Operation shape for `DeleteDestination`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_destination`](crate::client::Client::delete_destination).
///
/// See [`crate::client::fluent_builders::DeleteDestination`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteDestination {
    _private: (),
}
impl DeleteDestination {
    /// Creates a new builder-style object to manufacture [`DeleteDestinationInput`](crate::input::DeleteDestinationInput).
    pub fn builder() -> crate::input::delete_destination_input::Builder {
        crate::input::delete_destination_input::Builder::default()
    }
    /// Creates a new `DeleteDestination` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteDestination {
    type Output = std::result::Result<
        crate::output::DeleteDestinationOutput,
        crate::error::DeleteDestinationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_destination_error(response)
        } else {
            crate::operation_deser::parse_delete_destination_response(response)
        }
    }
}

/// Operation shape for `DeleteSite`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_site`](crate::client::Client::delete_site).
///
/// See [`crate::client::fluent_builders::DeleteSite`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteSite {
    _private: (),
}
impl DeleteSite {
    /// Creates a new builder-style object to manufacture [`DeleteSiteInput`](crate::input::DeleteSiteInput).
    pub fn builder() -> crate::input::delete_site_input::Builder {
        crate::input::delete_site_input::Builder::default()
    }
    /// Creates a new `DeleteSite` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteSite {
    type Output =
        std::result::Result<crate::output::DeleteSiteOutput, crate::error::DeleteSiteError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_site_error(response)
        } else {
            crate::operation_deser::parse_delete_site_response(response)
        }
    }
}

/// Operation shape for `DeleteWorker`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_worker`](crate::client::Client::delete_worker).
///
/// See [`crate::client::fluent_builders::DeleteWorker`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteWorker {
    _private: (),
}
impl DeleteWorker {
    /// Creates a new builder-style object to manufacture [`DeleteWorkerInput`](crate::input::DeleteWorkerInput).
    pub fn builder() -> crate::input::delete_worker_input::Builder {
        crate::input::delete_worker_input::Builder::default()
    }
    /// Creates a new `DeleteWorker` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteWorker {
    type Output =
        std::result::Result<crate::output::DeleteWorkerOutput, crate::error::DeleteWorkerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_worker_error(response)
        } else {
            crate::operation_deser::parse_delete_worker_response(response)
        }
    }
}

/// Operation shape for `DeleteWorkerFleet`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_worker_fleet`](crate::client::Client::delete_worker_fleet).
///
/// See [`crate::client::fluent_builders::DeleteWorkerFleet`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteWorkerFleet {
    _private: (),
}
impl DeleteWorkerFleet {
    /// Creates a new builder-style object to manufacture [`DeleteWorkerFleetInput`](crate::input::DeleteWorkerFleetInput).
    pub fn builder() -> crate::input::delete_worker_fleet_input::Builder {
        crate::input::delete_worker_fleet_input::Builder::default()
    }
    /// Creates a new `DeleteWorkerFleet` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteWorkerFleet {
    type Output = std::result::Result<
        crate::output::DeleteWorkerFleetOutput,
        crate::error::DeleteWorkerFleetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_worker_fleet_error(response)
        } else {
            crate::operation_deser::parse_delete_worker_fleet_response(response)
        }
    }
}

/// Operation shape for `GetDestination`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_destination`](crate::client::Client::get_destination).
///
/// See [`crate::client::fluent_builders::GetDestination`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetDestination {
    _private: (),
}
impl GetDestination {
    /// Creates a new builder-style object to manufacture [`GetDestinationInput`](crate::input::GetDestinationInput).
    pub fn builder() -> crate::input::get_destination_input::Builder {
        crate::input::get_destination_input::Builder::default()
    }
    /// Creates a new `GetDestination` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDestination {
    type Output =
        std::result::Result<crate::output::GetDestinationOutput, crate::error::GetDestinationError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_destination_error(response)
        } else {
            crate::operation_deser::parse_get_destination_response(response)
        }
    }
}

/// Operation shape for `GetSite`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_site`](crate::client::Client::get_site).
///
/// See [`crate::client::fluent_builders::GetSite`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetSite {
    _private: (),
}
impl GetSite {
    /// Creates a new builder-style object to manufacture [`GetSiteInput`](crate::input::GetSiteInput).
    pub fn builder() -> crate::input::get_site_input::Builder {
        crate::input::get_site_input::Builder::default()
    }
    /// Creates a new `GetSite` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSite {
    type Output = std::result::Result<crate::output::GetSiteOutput, crate::error::GetSiteError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_site_error(response)
        } else {
            crate::operation_deser::parse_get_site_response(response)
        }
    }
}

/// Operation shape for `GetWorker`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_worker`](crate::client::Client::get_worker).
///
/// See [`crate::client::fluent_builders::GetWorker`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetWorker {
    _private: (),
}
impl GetWorker {
    /// Creates a new builder-style object to manufacture [`GetWorkerInput`](crate::input::GetWorkerInput).
    pub fn builder() -> crate::input::get_worker_input::Builder {
        crate::input::get_worker_input::Builder::default()
    }
    /// Creates a new `GetWorker` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetWorker {
    type Output = std::result::Result<crate::output::GetWorkerOutput, crate::error::GetWorkerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_worker_error(response)
        } else {
            crate::operation_deser::parse_get_worker_response(response)
        }
    }
}

/// Operation shape for `GetWorkerFleet`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_worker_fleet`](crate::client::Client::get_worker_fleet).
///
/// See [`crate::client::fluent_builders::GetWorkerFleet`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetWorkerFleet {
    _private: (),
}
impl GetWorkerFleet {
    /// Creates a new builder-style object to manufacture [`GetWorkerFleetInput`](crate::input::GetWorkerFleetInput).
    pub fn builder() -> crate::input::get_worker_fleet_input::Builder {
        crate::input::get_worker_fleet_input::Builder::default()
    }
    /// Creates a new `GetWorkerFleet` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetWorkerFleet {
    type Output =
        std::result::Result<crate::output::GetWorkerFleetOutput, crate::error::GetWorkerFleetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_worker_fleet_error(response)
        } else {
            crate::operation_deser::parse_get_worker_fleet_response(response)
        }
    }
}

/// Operation shape for `ListDestinations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_destinations`](crate::client::Client::list_destinations).
///
/// See [`crate::client::fluent_builders::ListDestinations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListDestinations {
    _private: (),
}
impl ListDestinations {
    /// Creates a new builder-style object to manufacture [`ListDestinationsInput`](crate::input::ListDestinationsInput).
    pub fn builder() -> crate::input::list_destinations_input::Builder {
        crate::input::list_destinations_input::Builder::default()
    }
    /// Creates a new `ListDestinations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDestinations {
    type Output = std::result::Result<
        crate::output::ListDestinationsOutput,
        crate::error::ListDestinationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_destinations_error(response)
        } else {
            crate::operation_deser::parse_list_destinations_response(response)
        }
    }
}

/// Operation shape for `ListSites`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_sites`](crate::client::Client::list_sites).
///
/// See [`crate::client::fluent_builders::ListSites`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListSites {
    _private: (),
}
impl ListSites {
    /// Creates a new builder-style object to manufacture [`ListSitesInput`](crate::input::ListSitesInput).
    pub fn builder() -> crate::input::list_sites_input::Builder {
        crate::input::list_sites_input::Builder::default()
    }
    /// Creates a new `ListSites` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSites {
    type Output = std::result::Result<crate::output::ListSitesOutput, crate::error::ListSitesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_sites_error(response)
        } else {
            crate::operation_deser::parse_list_sites_response(response)
        }
    }
}

/// Operation shape for `ListWorkerFleets`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_worker_fleets`](crate::client::Client::list_worker_fleets).
///
/// See [`crate::client::fluent_builders::ListWorkerFleets`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListWorkerFleets {
    _private: (),
}
impl ListWorkerFleets {
    /// Creates a new builder-style object to manufacture [`ListWorkerFleetsInput`](crate::input::ListWorkerFleetsInput).
    pub fn builder() -> crate::input::list_worker_fleets_input::Builder {
        crate::input::list_worker_fleets_input::Builder::default()
    }
    /// Creates a new `ListWorkerFleets` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListWorkerFleets {
    type Output = std::result::Result<
        crate::output::ListWorkerFleetsOutput,
        crate::error::ListWorkerFleetsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_worker_fleets_error(response)
        } else {
            crate::operation_deser::parse_list_worker_fleets_response(response)
        }
    }
}

/// Operation shape for `ListWorkers`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_workers`](crate::client::Client::list_workers).
///
/// See [`crate::client::fluent_builders::ListWorkers`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListWorkers {
    _private: (),
}
impl ListWorkers {
    /// Creates a new builder-style object to manufacture [`ListWorkersInput`](crate::input::ListWorkersInput).
    pub fn builder() -> crate::input::list_workers_input::Builder {
        crate::input::list_workers_input::Builder::default()
    }
    /// Creates a new `ListWorkers` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListWorkers {
    type Output =
        std::result::Result<crate::output::ListWorkersOutput, crate::error::ListWorkersError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_workers_error(response)
        } else {
            crate::operation_deser::parse_list_workers_response(response)
        }
    }
}

/// Operation shape for `UpdateDestination`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_destination`](crate::client::Client::update_destination).
///
/// See [`crate::client::fluent_builders::UpdateDestination`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateDestination {
    _private: (),
}
impl UpdateDestination {
    /// Creates a new builder-style object to manufacture [`UpdateDestinationInput`](crate::input::UpdateDestinationInput).
    pub fn builder() -> crate::input::update_destination_input::Builder {
        crate::input::update_destination_input::Builder::default()
    }
    /// Creates a new `UpdateDestination` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateDestination {
    type Output = std::result::Result<
        crate::output::UpdateDestinationOutput,
        crate::error::UpdateDestinationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_destination_error(response)
        } else {
            crate::operation_deser::parse_update_destination_response(response)
        }
    }
}

/// Operation shape for `UpdateSite`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_site`](crate::client::Client::update_site).
///
/// See [`crate::client::fluent_builders::UpdateSite`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateSite {
    _private: (),
}
impl UpdateSite {
    /// Creates a new builder-style object to manufacture [`UpdateSiteInput`](crate::input::UpdateSiteInput).
    pub fn builder() -> crate::input::update_site_input::Builder {
        crate::input::update_site_input::Builder::default()
    }
    /// Creates a new `UpdateSite` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateSite {
    type Output =
        std::result::Result<crate::output::UpdateSiteOutput, crate::error::UpdateSiteError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_site_error(response)
        } else {
            crate::operation_deser::parse_update_site_response(response)
        }
    }
}

/// Operation shape for `UpdateWorker`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_worker`](crate::client::Client::update_worker).
///
/// See [`crate::client::fluent_builders::UpdateWorker`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateWorker {
    _private: (),
}
impl UpdateWorker {
    /// Creates a new builder-style object to manufacture [`UpdateWorkerInput`](crate::input::UpdateWorkerInput).
    pub fn builder() -> crate::input::update_worker_input::Builder {
        crate::input::update_worker_input::Builder::default()
    }
    /// Creates a new `UpdateWorker` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateWorker {
    type Output =
        std::result::Result<crate::output::UpdateWorkerOutput, crate::error::UpdateWorkerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_worker_error(response)
        } else {
            crate::operation_deser::parse_update_worker_response(response)
        }
    }
}

/// Operation shape for `UpdateWorkerFleet`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_worker_fleet`](crate::client::Client::update_worker_fleet).
///
/// See [`crate::client::fluent_builders::UpdateWorkerFleet`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateWorkerFleet {
    _private: (),
}
impl UpdateWorkerFleet {
    /// Creates a new builder-style object to manufacture [`UpdateWorkerFleetInput`](crate::input::UpdateWorkerFleetInput).
    pub fn builder() -> crate::input::update_worker_fleet_input::Builder {
        crate::input::update_worker_fleet_input::Builder::default()
    }
    /// Creates a new `UpdateWorkerFleet` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateWorkerFleet {
    type Output = std::result::Result<
        crate::output::UpdateWorkerFleetOutput,
        crate::error::UpdateWorkerFleetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_worker_fleet_error(response)
        } else {
            crate::operation_deser::parse_update_worker_fleet_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
