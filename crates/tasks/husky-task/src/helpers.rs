use crate::*;

pub type TaskDevLinkTime<Task> = <<Task as IsTask>::DevAscension as IsDevAscension>::Linktime;
pub type TaskDevLinkageImpl<Task> =
    <<<Task as IsTask>::DevAscension as IsDevAscension>::Linktime as IsLinktime>::LinkageImpl;
pub type TaskDevRuntimeSpecificConfig<Task> =
    <<Task as IsTask>::DevAscension as IsDevAscension>::RuntimeSpecificConfig;
pub type DevRuntimeStorage<Task> =
    <<Task as IsTask>::DevAscension as IsDevAscension>::RuntimeStorage;
pub type TaskDevBasePoint<Task> = <<Task as IsTask>::DevAscension as IsDevAscension>::BasePoint;
pub type TaskDevAscension<Task> = <Task as IsTask>::DevAscension;
pub type TaskValue<Task> = <TaskDevAscension<Task> as IsDevAscension>::Value;
pub type TaskTraceProtocol<Task> = <TaskDevAscension<Task> as IsDevAscension>::TraceProtocol;
