[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::major::connected_components`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`~ [] mnist_classifier::connected_component::ConnectedComponent`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`~ mnist_classifier::connected_component::ConnectedComponent`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`core::num::f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`~ [] mnist_classifier::raw_contour::RawContour`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`~ mnist_classifier::raw_contour::RawContour`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`~ mnist_classifier::line_segment_sketch::LineSegmentSketch`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`~ [] mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
        ),
    ),
]