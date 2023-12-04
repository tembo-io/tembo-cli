/*
 * Tembo Cloud
 *
 * Platform API for Tembo Cloud             </br>             </br>             To find a Tembo Data API, please find it here:             </br>             </br>             [AWS US East 1](https://api.data-1.use1.tembo.io/swagger-ui/)             
 *
 * The version of the OpenAPI document: v1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EnvVarRef {
    #[serde(rename = "ReadOnlyConnection")]
    ReadOnlyConnection,
    #[serde(rename = "ReadWriteConnection")]
    ReadWriteConnection,

}

impl ToString for EnvVarRef {
    fn to_string(&self) -> String {
        match self {
            Self::ReadOnlyConnection => String::from("ReadOnlyConnection"),
            Self::ReadWriteConnection => String::from("ReadWriteConnection"),
        }
    }
}

impl Default for EnvVarRef {
    fn default() -> EnvVarRef {
        Self::ReadOnlyConnection
    }
}




