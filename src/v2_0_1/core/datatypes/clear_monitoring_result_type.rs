use crate::v2_0_1::core::enumerations::clear_monitoring_status_enum_type::ClearMonitoringStatusEnumType;

use super::status_info_type::StatusInfoType;

/// ClearMonitoringResultType is used by: ClearVariableMonitoringResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearMonitoringResultType {
    pub status: ClearMonitoringStatusEnumType,
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
