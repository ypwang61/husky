use crate::*;
use husky_task_interface::pedestal::IsPedestal;
use husky_visual_protocol::synchrotron::VisualSynchrotron;
use ui::ui::IsUi;

use self::view::action::TraceViewActionBuffer;

pub trait PedestalUi<Ui: IsUi>: IsPedestal {
    fn pedestal_ui<TraceProtocol>(
        self,
        ui: &mut Ui,
        pedestal_buffer: &mut Self::UiBuffer,
        action_buffer: &mut TraceViewActionBuffer<TraceProtocol>,
    ) where
        TraceProtocol: IsTraceProtocol<Pedestal = Self>;
}

pub type TracePedestalUiBuffer<TraceProtocol> =
    <<TraceProtocol as IsTraceProtocol>::Pedestal as IsPedestal>::UiBuffer;
