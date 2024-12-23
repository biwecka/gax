// Resource Type ///////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[deprecated]
pub struct ResourceTypeId(pub String);
pub type ResourceTypeRef = ResourceTypeId;

impl From<String> for ResourceTypeId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for ResourceTypeId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[derive(Clone, Debug)]
#[deprecated]
pub struct ResourceType {
    pub id: ResourceTypeId,
    pub name: String,
}

impl From<crate::parser::instances::resources::ResourceType> for ResourceType {
    fn from(value: crate::parser::instances::resources::ResourceType) -> Self {
        let id = ResourceTypeId(value.id);
        let name = value.name;

        Self { id, name }
    }
}

// Resource Group //////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[deprecated]
pub struct ResourceGroupId(pub String);
pub type ResourceGroupRef = ResourceGroupId;

impl From<String> for ResourceGroupId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for ResourceGroupId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[derive(Clone, Debug)]
#[deprecated]
pub struct ResourceGroup {
    pub id: ResourceGroupId,
    pub name: String,

    pub resource_type: ResourceTypeRef,
}

impl From<crate::parser::instances::resources::ResourceGroup>
    for ResourceGroup
{
    fn from(value: crate::parser::instances::resources::ResourceGroup) -> Self {
        let id = ResourceGroupId(value.id);
        let name = value.name;

        let resource_type = ResourceTypeId(value.resource_type.reference);

        Self { id, name, resource_type }
    }
}

// Resource ////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[deprecated]
pub struct ResourceId(pub String);
pub type ResourceRef = ResourceId;

impl From<String> for ResourceId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for ResourceId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[derive(Clone, Debug)]
#[deprecated]
pub struct Resource {
    pub id: ResourceId,
    pub name: String,
    pub resource_type: ResourceTypeRef,
    pub resource_groups: Vec<ResourceGroupRef>,
}

impl From<crate::parser::instances::resources::Resource> for Resource {
    fn from(value: crate::parser::instances::resources::Resource) -> Self {
        let id = ResourceId(value.id);
        let name = value.name;
        let resource_type = ResourceTypeId(value.resource_type.reference);
        let resource_groups = value
            .resource_groups
            .map(|refs| {
                refs.list
                    .into_iter()
                    .map(|x| ResourceGroupId(x.reference))
                    .collect()
            })
            .unwrap_or_default();

        Self { id, name, resource_type, resource_groups }
    }
}

////////////////////////////////////////////////////////////////////////////////
