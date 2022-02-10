use feature::FeatureBlock;

use crate::*;

pub trait AskCompileTime {
    fn compile_time(&self, version: usize) -> &HuskyLangCompileTime;
}

#[salsa::query_group(RuntimeQueryGroupStorage)]
pub trait RuntimeQueryGroup: AskCompileTime + AllocateTrace {
    #[salsa::input]
    fn package_main(&self) -> FilePtr;

    #[salsa::input]
    fn version(&self) -> usize;

    fn subtraces(&self, id: usize) -> Arc<Vec<Arc<Trace>>>;
    fn root_traces(&self) -> Arc<Vec<Arc<Trace>>>;
}

pub fn root_traces(this: &dyn RuntimeQueryGroup) -> Arc<Vec<Arc<Trace>>> {
    let compile_time = this.compile_time(this.version());
    let package_main = this.package_main();
    Arc::new(vec![this.new_trace(
        None,
        0,
        TraceKind::Main {
            main_file: package_main,
            feature_block: compile_time.main_feature_block(package_main).unwrap(),
        },
    )])
}

pub fn subtraces(this: &dyn RuntimeQueryGroup, id: usize) -> Arc<Vec<Arc<Trace>>> {
    let trace = this.trace(id);
    match trace.kind {
        TraceKind::Mock { ref tokens } => trace::mock::subtraces(id),
        TraceKind::Main {
            main_file,
            ref feature_block,
        } => Arc::new(
            this.trace_allocator()
                .feature_block_subtraces(Some(trace.id), feature_block),
        ),
        TraceKind::Stmt(ref stmt) => this
            .trace_allocator()
            .feature_stmt_subtraces(trace.id, stmt),
        TraceKind::Expr(ref expr) => {
            this.trace_allocator()
                .feature_expr_subtraces(trace.id, trace.indent, expr)
        }
        TraceKind::Branch(ref branch) => this.trace_allocator().feature_branch_subtraces(
            trace.id,
            trace.indent,
            branch,
            this.trace_allocator(),
        ),
        TraceKind::Condition(ref cond) => {
            this.trace_allocator()
                .feature_expr_subtraces(trace.id, trace.indent, cond)
        }
    }
}
