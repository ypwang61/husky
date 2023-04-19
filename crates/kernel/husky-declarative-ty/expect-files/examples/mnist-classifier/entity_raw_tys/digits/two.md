[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::two::two_match`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`mnist_classifier::fermi::FermiMatchResult`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Fp(~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent) -> core::option::Option core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`core::option::Option mnist::MnistLabel`),
        ),
    ),
]