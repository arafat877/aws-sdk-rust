// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AssociateNode`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`associate_node`](crate::client::Client::associate_node).
///
/// See [`crate::client::fluent_builders::AssociateNode`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssociateNode {
    _private: (),
}
impl AssociateNode {
    /// Creates a new builder-style object to manufacture [`AssociateNodeInput`](crate::input::AssociateNodeInput).
    pub fn builder() -> crate::input::associate_node_input::Builder {
        crate::input::associate_node_input::Builder::default()
    }
    /// Creates a new `AssociateNode` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateNode {
    type Output =
        std::result::Result<crate::output::AssociateNodeOutput, crate::error::AssociateNodeError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_associate_node_error(response)
        } else {
            crate::operation_deser::parse_associate_node_response(response)
        }
    }
}

/// Operation shape for `CreateBackup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_backup`](crate::client::Client::create_backup).
///
/// See [`crate::client::fluent_builders::CreateBackup`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateBackup {
    _private: (),
}
impl CreateBackup {
    /// Creates a new builder-style object to manufacture [`CreateBackupInput`](crate::input::CreateBackupInput).
    pub fn builder() -> crate::input::create_backup_input::Builder {
        crate::input::create_backup_input::Builder::default()
    }
    /// Creates a new `CreateBackup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateBackup {
    type Output =
        std::result::Result<crate::output::CreateBackupOutput, crate::error::CreateBackupError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_backup_error(response)
        } else {
            crate::operation_deser::parse_create_backup_response(response)
        }
    }
}

/// Operation shape for `CreateServer`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_server`](crate::client::Client::create_server).
///
/// See [`crate::client::fluent_builders::CreateServer`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateServer {
    _private: (),
}
impl CreateServer {
    /// Creates a new builder-style object to manufacture [`CreateServerInput`](crate::input::CreateServerInput).
    pub fn builder() -> crate::input::create_server_input::Builder {
        crate::input::create_server_input::Builder::default()
    }
    /// Creates a new `CreateServer` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateServer {
    type Output =
        std::result::Result<crate::output::CreateServerOutput, crate::error::CreateServerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_server_error(response)
        } else {
            crate::operation_deser::parse_create_server_response(response)
        }
    }
}

/// Operation shape for `DeleteBackup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_backup`](crate::client::Client::delete_backup).
///
/// See [`crate::client::fluent_builders::DeleteBackup`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteBackup {
    _private: (),
}
impl DeleteBackup {
    /// Creates a new builder-style object to manufacture [`DeleteBackupInput`](crate::input::DeleteBackupInput).
    pub fn builder() -> crate::input::delete_backup_input::Builder {
        crate::input::delete_backup_input::Builder::default()
    }
    /// Creates a new `DeleteBackup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteBackup {
    type Output =
        std::result::Result<crate::output::DeleteBackupOutput, crate::error::DeleteBackupError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_backup_error(response)
        } else {
            crate::operation_deser::parse_delete_backup_response(response)
        }
    }
}

/// Operation shape for `DeleteServer`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_server`](crate::client::Client::delete_server).
///
/// See [`crate::client::fluent_builders::DeleteServer`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteServer {
    _private: (),
}
impl DeleteServer {
    /// Creates a new builder-style object to manufacture [`DeleteServerInput`](crate::input::DeleteServerInput).
    pub fn builder() -> crate::input::delete_server_input::Builder {
        crate::input::delete_server_input::Builder::default()
    }
    /// Creates a new `DeleteServer` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteServer {
    type Output =
        std::result::Result<crate::output::DeleteServerOutput, crate::error::DeleteServerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_server_error(response)
        } else {
            crate::operation_deser::parse_delete_server_response(response)
        }
    }
}

/// Operation shape for `DescribeAccountAttributes`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_account_attributes`](crate::client::Client::describe_account_attributes).
///
/// See [`crate::client::fluent_builders::DescribeAccountAttributes`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeAccountAttributes {
    _private: (),
}
impl DescribeAccountAttributes {
    /// Creates a new builder-style object to manufacture [`DescribeAccountAttributesInput`](crate::input::DescribeAccountAttributesInput).
    pub fn builder() -> crate::input::describe_account_attributes_input::Builder {
        crate::input::describe_account_attributes_input::Builder::default()
    }
    /// Creates a new `DescribeAccountAttributes` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAccountAttributes {
    type Output = std::result::Result<
        crate::output::DescribeAccountAttributesOutput,
        crate::error::DescribeAccountAttributesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_account_attributes_error(response)
        } else {
            crate::operation_deser::parse_describe_account_attributes_response(response)
        }
    }
}

/// Operation shape for `DescribeBackups`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_backups`](crate::client::Client::describe_backups).
///
/// See [`crate::client::fluent_builders::DescribeBackups`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeBackups {
    _private: (),
}
impl DescribeBackups {
    /// Creates a new builder-style object to manufacture [`DescribeBackupsInput`](crate::input::DescribeBackupsInput).
    pub fn builder() -> crate::input::describe_backups_input::Builder {
        crate::input::describe_backups_input::Builder::default()
    }
    /// Creates a new `DescribeBackups` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeBackups {
    type Output = std::result::Result<
        crate::output::DescribeBackupsOutput,
        crate::error::DescribeBackupsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_backups_error(response)
        } else {
            crate::operation_deser::parse_describe_backups_response(response)
        }
    }
}

/// Operation shape for `DescribeEvents`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_events`](crate::client::Client::describe_events).
///
/// See [`crate::client::fluent_builders::DescribeEvents`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeEvents {
    _private: (),
}
impl DescribeEvents {
    /// Creates a new builder-style object to manufacture [`DescribeEventsInput`](crate::input::DescribeEventsInput).
    pub fn builder() -> crate::input::describe_events_input::Builder {
        crate::input::describe_events_input::Builder::default()
    }
    /// Creates a new `DescribeEvents` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeEvents {
    type Output =
        std::result::Result<crate::output::DescribeEventsOutput, crate::error::DescribeEventsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_events_error(response)
        } else {
            crate::operation_deser::parse_describe_events_response(response)
        }
    }
}

/// Operation shape for `DescribeNodeAssociationStatus`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_node_association_status`](crate::client::Client::describe_node_association_status).
///
/// See [`crate::client::fluent_builders::DescribeNodeAssociationStatus`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeNodeAssociationStatus {
    _private: (),
}
impl DescribeNodeAssociationStatus {
    /// Creates a new builder-style object to manufacture [`DescribeNodeAssociationStatusInput`](crate::input::DescribeNodeAssociationStatusInput).
    pub fn builder() -> crate::input::describe_node_association_status_input::Builder {
        crate::input::describe_node_association_status_input::Builder::default()
    }
    /// Creates a new `DescribeNodeAssociationStatus` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeNodeAssociationStatus {
    type Output = std::result::Result<
        crate::output::DescribeNodeAssociationStatusOutput,
        crate::error::DescribeNodeAssociationStatusError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_node_association_status_error(response)
        } else {
            crate::operation_deser::parse_describe_node_association_status_response(response)
        }
    }
}

/// Operation shape for `DescribeServers`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_servers`](crate::client::Client::describe_servers).
///
/// See [`crate::client::fluent_builders::DescribeServers`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeServers {
    _private: (),
}
impl DescribeServers {
    /// Creates a new builder-style object to manufacture [`DescribeServersInput`](crate::input::DescribeServersInput).
    pub fn builder() -> crate::input::describe_servers_input::Builder {
        crate::input::describe_servers_input::Builder::default()
    }
    /// Creates a new `DescribeServers` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeServers {
    type Output = std::result::Result<
        crate::output::DescribeServersOutput,
        crate::error::DescribeServersError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_servers_error(response)
        } else {
            crate::operation_deser::parse_describe_servers_response(response)
        }
    }
}

/// Operation shape for `DisassociateNode`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`disassociate_node`](crate::client::Client::disassociate_node).
///
/// See [`crate::client::fluent_builders::DisassociateNode`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DisassociateNode {
    _private: (),
}
impl DisassociateNode {
    /// Creates a new builder-style object to manufacture [`DisassociateNodeInput`](crate::input::DisassociateNodeInput).
    pub fn builder() -> crate::input::disassociate_node_input::Builder {
        crate::input::disassociate_node_input::Builder::default()
    }
    /// Creates a new `DisassociateNode` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateNode {
    type Output = std::result::Result<
        crate::output::DisassociateNodeOutput,
        crate::error::DisassociateNodeError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_disassociate_node_error(response)
        } else {
            crate::operation_deser::parse_disassociate_node_response(response)
        }
    }
}

/// Operation shape for `ExportServerEngineAttribute`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`export_server_engine_attribute`](crate::client::Client::export_server_engine_attribute).
///
/// See [`crate::client::fluent_builders::ExportServerEngineAttribute`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ExportServerEngineAttribute {
    _private: (),
}
impl ExportServerEngineAttribute {
    /// Creates a new builder-style object to manufacture [`ExportServerEngineAttributeInput`](crate::input::ExportServerEngineAttributeInput).
    pub fn builder() -> crate::input::export_server_engine_attribute_input::Builder {
        crate::input::export_server_engine_attribute_input::Builder::default()
    }
    /// Creates a new `ExportServerEngineAttribute` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExportServerEngineAttribute {
    type Output = std::result::Result<
        crate::output::ExportServerEngineAttributeOutput,
        crate::error::ExportServerEngineAttributeError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_export_server_engine_attribute_error(response)
        } else {
            crate::operation_deser::parse_export_server_engine_attribute_response(response)
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

/// Operation shape for `RestoreServer`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`restore_server`](crate::client::Client::restore_server).
///
/// See [`crate::client::fluent_builders::RestoreServer`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RestoreServer {
    _private: (),
}
impl RestoreServer {
    /// Creates a new builder-style object to manufacture [`RestoreServerInput`](crate::input::RestoreServerInput).
    pub fn builder() -> crate::input::restore_server_input::Builder {
        crate::input::restore_server_input::Builder::default()
    }
    /// Creates a new `RestoreServer` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RestoreServer {
    type Output =
        std::result::Result<crate::output::RestoreServerOutput, crate::error::RestoreServerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_restore_server_error(response)
        } else {
            crate::operation_deser::parse_restore_server_response(response)
        }
    }
}

/// Operation shape for `StartMaintenance`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_maintenance`](crate::client::Client::start_maintenance).
///
/// See [`crate::client::fluent_builders::StartMaintenance`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartMaintenance {
    _private: (),
}
impl StartMaintenance {
    /// Creates a new builder-style object to manufacture [`StartMaintenanceInput`](crate::input::StartMaintenanceInput).
    pub fn builder() -> crate::input::start_maintenance_input::Builder {
        crate::input::start_maintenance_input::Builder::default()
    }
    /// Creates a new `StartMaintenance` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartMaintenance {
    type Output = std::result::Result<
        crate::output::StartMaintenanceOutput,
        crate::error::StartMaintenanceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_maintenance_error(response)
        } else {
            crate::operation_deser::parse_start_maintenance_response(response)
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

/// Operation shape for `UpdateServer`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_server`](crate::client::Client::update_server).
///
/// See [`crate::client::fluent_builders::UpdateServer`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateServer {
    _private: (),
}
impl UpdateServer {
    /// Creates a new builder-style object to manufacture [`UpdateServerInput`](crate::input::UpdateServerInput).
    pub fn builder() -> crate::input::update_server_input::Builder {
        crate::input::update_server_input::Builder::default()
    }
    /// Creates a new `UpdateServer` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateServer {
    type Output =
        std::result::Result<crate::output::UpdateServerOutput, crate::error::UpdateServerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_server_error(response)
        } else {
            crate::operation_deser::parse_update_server_response(response)
        }
    }
}

/// Operation shape for `UpdateServerEngineAttributes`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_server_engine_attributes`](crate::client::Client::update_server_engine_attributes).
///
/// See [`crate::client::fluent_builders::UpdateServerEngineAttributes`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateServerEngineAttributes {
    _private: (),
}
impl UpdateServerEngineAttributes {
    /// Creates a new builder-style object to manufacture [`UpdateServerEngineAttributesInput`](crate::input::UpdateServerEngineAttributesInput).
    pub fn builder() -> crate::input::update_server_engine_attributes_input::Builder {
        crate::input::update_server_engine_attributes_input::Builder::default()
    }
    /// Creates a new `UpdateServerEngineAttributes` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateServerEngineAttributes {
    type Output = std::result::Result<
        crate::output::UpdateServerEngineAttributesOutput,
        crate::error::UpdateServerEngineAttributesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_server_engine_attributes_error(response)
        } else {
            crate::operation_deser::parse_update_server_engine_attributes_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
