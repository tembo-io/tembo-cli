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
pub struct MiddlewareOneOf2 {
    #[serde(rename = "replacePathRegex")]
    pub replace_path_regex: Box<crate::models::ReplacePathRegexConfig>,
}

impl MiddlewareOneOf2 {
    pub fn new(replace_path_regex: crate::models::ReplacePathRegexConfig) -> MiddlewareOneOf2 {
        MiddlewareOneOf2 {
            replace_path_regex: Box::new(replace_path_regex),
        }
    }
}

