#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum CertificateActionEnumType {
    Install,
    Update,
}
