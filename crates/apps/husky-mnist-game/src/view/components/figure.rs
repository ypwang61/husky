use crate::{channel::MnistChannel, op::history::OpTime, trace::Trace, MnistDb};

use husky_graphics2d_visual_protocol::figure::Graphics2dFigure;
use husky_standard_devsoul_interface::pedestal::StandardPedestal;
use husky_trace_protocol::figure::IsFigure;
use husky_visual_protocol::synchrotron::VisualSynchrotron;

pub type Figure = Graphics2dFigure<StandardPedestal>;

impl MnistChannel {
    pub(in super::super) fn figure<'a>(
        &'a self,
        pedestal: StandardPedestal,
        op_time: OpTime,
        db: &'a MnistDb,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Figure {
        let accompanyings_except_followed = self.accompanyings_except_followed();
        todo!()
        // match pedestal {
        //     StandardPedestal::Specific(input_id) => Figure::new_specific(
        //         None,
        //         &accompanyings_except_followed,
        //         |ki_repr_interface, _visual_synchrotron| {
        //             let trace: Trace = ki_repr_interface.into();
        //             trace.visual(db, input_id, op_time)
        //         },
        //         visual_synchrotron,
        //     ),
        //     StandardPedestal::Generic => todo!(),
        // }
    }

    pub(in super::super) fn accompanyings_except_followed(
        &self,
    ) -> Vec<(
        husky_trace_protocol::id::TraceId,
        husky_devsoul_interface::ki_repr::KiReprInterface,
    )> {
        self.trace_selection()
            .set()
            .iter()
            .map(|trace| (trace.into(), trace.into()))
            .collect::<Vec<_>>()
    }
}
