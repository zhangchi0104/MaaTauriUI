use std::ffi::CString;

pub enum CoreOption {
    TouchMode(TouchModeOption),
    DeploymentWithPause(bool),
}
pub enum TouchModeOption {
    MaaTouch,
    Adb,
    MiniTouch,
}

impl CoreOption {
    pub fn key(&self) -> i32 {
        match self {
            CoreOption::TouchMode(_) => 2,
            CoreOption::DeploymentWithPause(_) => 3,
        }
    }
    pub fn value(&self) -> String {
        let val = match self {
            CoreOption::TouchMode(mode) => match mode {
                TouchModeOption::MaaTouch => "maatouch",
                TouchModeOption::Adb => "adb",
                TouchModeOption::MiniTouch => "minitouch",
            },
            CoreOption::DeploymentWithPause(should_pause) => {
                if *should_pause {
                    "1"
                } else {
                    "0"
                }
            }
        };
        String::from(val)
    }
    pub fn value_cstr(&self) -> CString {
        let val = self.value();
        CString::new(val).expect("CString::new failed")
    }
}
