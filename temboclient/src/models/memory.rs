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
pub enum Memory {
    #[serde(rename = "1Gi")]
    Variant1Gi,
    #[serde(rename = "2Gi")]
    Variant2Gi,
    #[serde(rename = "4Gi")]
    Variant4Gi,
    #[serde(rename = "8Gi")]
    Variant8Gi,
    #[serde(rename = "16Gi")]
    Variant16Gi,
    #[serde(rename = "32Gi")]
    Variant32Gi,

}

impl ToString for Memory {
    fn to_string(&self) -> String {
        match self {
            Self::Variant1Gi => String::from("1Gi"),
            Self::Variant2Gi => String::from("2Gi"),
            Self::Variant4Gi => String::from("4Gi"),
            Self::Variant8Gi => String::from("8Gi"),
            Self::Variant16Gi => String::from("16Gi"),
            Self::Variant32Gi => String::from("32Gi"),
        }
    }
}

impl std::str::FromStr for Memory {
    type Err = ();

    fn from_str(input: &str) -> core::result::Result<Memory, Self::Err> {
        match input {
            "1Gi"  => Ok(Memory::Variant1Gi),
            "2Gi"  => Ok(Memory::Variant2Gi),
            "4Gi"  => Ok(Memory::Variant4Gi),
            "8Gi" => Ok(Memory::Variant8Gi),
            "16Gi" => Ok(Memory::Variant16Gi),
            "32Gi" => Ok(Memory::Variant32Gi),
            _      => Err(()),
        }
    }
}

impl Default for Memory {
    fn default() -> Memory {
        Self::Variant1Gi
    }
}



