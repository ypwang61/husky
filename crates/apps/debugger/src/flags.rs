use std::path::PathBuf;

xflags::xflags! {
    cmd husky-lang-debugger-command
        required dir: PathBuf
    {
        optional -v, --verbose
        optional --input-id input_id: String
        optional --mode mode: String
        optional -c, --compile
    }
}
