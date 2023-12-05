/*
 * Tembo Cloud
 *
 * Platform API for Tembo Cloud             </br>             </br>             To find a Tembo Data API, please find it here:             </br>             </br>             [AWS US East 1](https://api.data-1.use1.tembo.io/swagger-ui/)             
 *
 * The version of the OpenAPI document: v1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvVar {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value: Option<Option<String>>,
    #[serde(rename = "valueFromPlatform", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value_from_platform: Option<Option<crate::models::EnvVarRef>>,
}

impl EnvVar {
    pub fn new(name: String) -> EnvVar {
        EnvVar {
            name,
            value: None,
            value_from_platform: None,
        }
    }
}


