// Helper Macro ////////////////////////////////////////////////////////////////
macro_rules! load_asset {
    ($file:expr) => {
        include_str!(concat!("../../../../assets/", $file)).to_owned()
    };
}

// Instances Enum //////////////////////////////////////////////////////////////
structstruck::strike!(
    pub enum Instances {
        X2014a(pub enum X2014a {
            Abramson15,
            All11,
        }),
    }
);

impl Instances {
    pub fn xml(&self) -> String {
        match self {
            Self::X2014a(val) => match val {
                X2014a::Abramson15 => load_asset!("xhstt-2014a/Abramson15.xml"),
                X2014a::All11 => load_asset!("xhstt-2014a/All11.xml"),
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
