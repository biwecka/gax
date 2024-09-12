// Helper Macro ////////////////////////////////////////////////////////////////
macro_rules! load_asset {
    ($file:expr) => {
        include_str!(concat!("../../../assets/", $file)).to_owned()
    };
}

// Instances Enum //////////////////////////////////////////////////////////////
structstruck::strike!(
    #[strikethrough[derive(enum_iterator::Sequence)]]
    pub enum Archives {
        X2014(pub enum {
            AuBg98,
            AuSa96,
            AuTe99,
            BrSa00,
            BrSm00,
            BrSn00,
            DkFg12,
            DkHg12,
            DkVg09,
            EsSS08,
            FiMp06,
            FiPb98,
            FiWp06,
            GrH197,
            GrP310,
            GrPa08,
            ItI496,
            KsPr11,
            NlKp03,
            NlKp05,
            NlKp09,
            UkSp06,
            UsWs09,
            ZaLw09,
            ZaWd09,
        }),

        X2014a(pub enum {
            Abramson15,
            All11,
            All15,
            Hdtt4,
            Hdtt5,
            Hdtt6,
            Hdtt7,
            Hdtt8,
            Sudoku4x4,
        }),
    }
);

impl Archives {
    pub fn xml(&self) -> String {
        match self {
            Self::X2014(val) => match val {
                X2014::AuBg98 => load_asset!("xhstt-2014/AU-BG-98.xml"),
                X2014::AuSa96 => load_asset!("xhstt-2014/AU-SA-96.xml"),
                X2014::AuTe99 => load_asset!("xhstt-2014/AU-TE-99.xml"),
                X2014::BrSa00 => load_asset!("xhstt-2014/BR-SA-00.xml"),
                X2014::BrSm00 => load_asset!("xhstt-2014/BR-SM-00.xml"),
                X2014::BrSn00 => load_asset!("xhstt-2014/BR-SN-00.xml"),
                X2014::DkFg12 => load_asset!("xhstt-2014/DK-FG-12.xml"),
                X2014::DkHg12 => load_asset!("xhstt-2014/DK-HG-12.xml"),
                X2014::DkVg09 => load_asset!("xhstt-2014/DK-VG-09.xml"),
                X2014::EsSS08 => load_asset!("xhstt-2014/ES-SS-08.xml"),
                X2014::FiMp06 => load_asset!("xhstt-2014/FI-MP-06.xml"),
                X2014::FiPb98 => load_asset!("xhstt-2014/FI-PB-98.xml"),
                X2014::FiWp06 => load_asset!("xhstt-2014/FI-WP-06.xml"),
                X2014::GrH197 => load_asset!("xhstt-2014/GR-H1-97.xml"),
                X2014::GrP310 => load_asset!("xhstt-2014/GR-P3-10.xml"),
                X2014::GrPa08 => load_asset!("xhstt-2014/GR-PA-08.xml"),
                X2014::ItI496 => load_asset!("xhstt-2014/IT-I4-96.xml"),
                X2014::KsPr11 => load_asset!("xhstt-2014/KS-PR-11.xml"),
                X2014::NlKp03 => load_asset!("xhstt-2014/NL-KP-03.xml"),
                X2014::NlKp05 => load_asset!("xhstt-2014/NL-KP-05.xml"),
                X2014::NlKp09 => load_asset!("xhstt-2014/NL-KP-09.xml"),
                X2014::UkSp06 => load_asset!("xhstt-2014/UK-SP-06.xml"),
                X2014::UsWs09 => load_asset!("xhstt-2014/US-WS-09.xml"),
                X2014::ZaLw09 => load_asset!("xhstt-2014/ZA-LW-09.xml"),
                X2014::ZaWd09 => load_asset!("xhstt-2014/ZA-WD-09.xml"),
            },

            Self::X2014a(val) => match val {
                X2014a::Abramson15 => load_asset!("xhstt-2014a/Abramson15.xml"),
                X2014a::All11 => load_asset!("xhstt-2014a/All11.xml"),
                X2014a::All15 => load_asset!("xhstt-2014a/All15.xml"),
                X2014a::Hdtt4 => load_asset!("xhstt-2014a/Hdtt4.xml"),
                X2014a::Hdtt5 => load_asset!("xhstt-2014a/Hdtt5.xml"),
                X2014a::Hdtt6 => load_asset!("xhstt-2014a/Hdtt6.xml"),
                X2014a::Hdtt7 => load_asset!("xhstt-2014a/Hdtt7.xml"),
                X2014a::Hdtt8 => load_asset!("xhstt-2014a/Hdtt8.xml"),
                X2014a::Sudoku4x4 => load_asset!("xhstt-2014a/Sudoku4x4.xml"),
            },
        }
    }

    pub fn all_xml() -> Vec<String> {
        enum_iterator::all::<Archives>().map(|x| x.xml()).collect()
    }
}

////////////////////////////////////////////////////////////////////////////////
