// Helper Macro ////////////////////////////////////////////////////////////////
macro_rules! load_asset {
    ($file:expr) => {
        include_str!(concat!("../../../../assets/", $file)).to_owned()
    };
}

// Instances Enum //////////////////////////////////////////////////////////////
structstruck::strike!(
    pub enum Instances {
        // X2014(pub enum {
        //     AustraliaBGHS98,
        // }),

        X2014a(pub enum {
            Abramson15,
            All11,
        }),
    }
);

impl Instances {
    pub fn xml(&self) -> String {
        match self {
            // Self::X2014(val) => match val {
            //     X2014::AustraliaBGHS98 => load_asset!("xhstt-2014/AustraliaBGHS98.xml")
            // },

            Self::X2014a(val) => match val {
                X2014a::Abramson15 => load_asset!("xhstt-2014a/Abramson15.xml"),
                X2014a::All11 => load_asset!("xhstt-2014a/All11.xml"),
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
