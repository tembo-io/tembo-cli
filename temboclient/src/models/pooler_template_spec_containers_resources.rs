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
pub struct PoolerTemplateSpecContainersResources {
    #[serde(rename = "claims", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub claims: Option<Option<Vec<crate::models::PoolerTemplateSpecContainersResourcesClaims>>>,
    #[serde(rename = "limits", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub limits: Option<Option<::std::collections::HashMap<String, crate::models::IntOrString>>>,
    #[serde(rename = "requests", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub requests: Option<Option<::std::collections::HashMap<String, crate::models::IntOrString>>>,
}

impl PoolerTemplateSpecContainersResources {
    pub fn new() -> PoolerTemplateSpecContainersResources {
        PoolerTemplateSpecContainersResources {
            claims: None,
            limits: None,
            requests: None,
        }
    }
}


