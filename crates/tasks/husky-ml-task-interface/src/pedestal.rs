#[cfg(feature = "egui")]
mod egui;

use super::InputId;
use super::*;
use husky_task_interface::pedestal::{IsPedestal, IsPedestalUiBuffer};

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MlPedestal {
    Specific(InputId),
    Generic,
}

impl Default for MlPedestal {
    fn default() -> Self {
        MlPedestal::Specific(InputId::from_index(0))
    }
}

impl IsPedestal for MlPedestal {
    type UiBuffer = MlPedestalUiBuffer;

    fn init_ui_buffer(self) -> Self::UiBuffer {
        MlPedestalUiBuffer {
            input_id_to_be: match self {
                MlPedestal::Specific(input_id) => input_id.index().to_string(),
                MlPedestal::Generic => "0".to_string(),
            },
            error: None,
        }
    }
}

impl MlPedestal {
    pub fn input_id(self) -> Option<InputId> {
        match self {
            MlPedestal::Specific(input_id) => Some(input_id),
            MlPedestal::Generic => None,
        }
    }
}

pub struct MlPedestalUiBuffer {
    input_id_to_be: String,
    error: Option<String>,
}

impl IsPedestalUiBuffer for MlPedestalUiBuffer {
    type Pedestal = MlPedestal;

    fn update(&mut self, pedestal: Self::Pedestal) {
        self.error = None;
        *self = pedestal.init_ui_buffer()
    }
}
