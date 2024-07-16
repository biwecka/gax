// Imports /////////////////////////////////////////////////////////////////////

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Resources {
    #[serde(rename = "ResourceTypes", skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<ResourceTypes>,

    #[serde(rename = "ResourceGroups", skip_serializing_if = "Option::is_none")]
    pub resource_groups: Option<ResourceGroups>,

    #[serde(rename = "Resource", default)]
    pub resources: Vec<Resource>,
}

// Sub-Structs /////////////////////////////////////////////////////////////////
structstruck::strike!(
    #[strikethrough[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]]
    pub struct ResourceTypes {
        #[serde(rename = "ResourceType", default)]
        pub list: Vec<pub struct ResourceType {
            // Attributes
            #[serde(rename = "@Id")]
            pub id: String,


            // Children
            #[serde(rename = "Name")]
            pub name: String,
        }>,
    }
);

structstruck::strike!(
    #[strikethrough[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]]
    pub struct ResourceGroups {
        #[serde(rename = "ResourceGroup", default)]
        pub list: Vec<pub struct ResourceGroup {
            // Attributes
            #[serde(rename = "@Id")]
            pub id: String,


            // Children
            #[serde(rename = "Name")]
            pub name: String,

            #[serde(rename = "ResourceType")]
            pub resource_type: pub struct ResourceTypeRef {

                #[serde(rename = "@Reference")]
                pub reference: String,
            }
        }>,
    }
);

structstruck::strike!(
    #[strikethrough[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]]
    pub struct Resource {
        // Attributes
        #[serde(rename = "@Id")]
        pub id: String,


        // Children
        #[serde(rename = "Name")]
        pub name: String,

        #[serde(rename = "ResourceType")]
        pub resource_type: ResourceTypeRef,

        #[serde(rename = "ResourceGroups", skip_serializing_if = "Option::is_none")]
        pub resource_groups: Option<pub struct ResourceGroupRefs {

            #[serde(rename = "ResourceGroup", default)]
            pub list: Vec<pub struct ResourceGroupRef {

                #[serde(rename = "@Reference")]
                pub reference: String,
            }>,
        }>,
    }
);

////////////////////////////////////////////////////////////////////////////////
