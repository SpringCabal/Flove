use spring::Float3;
use springcabal_control::camera::{View, place_camera};

const VIEW: View = View::spring(2384.9, 2.377)
    .pitch(-0.72166, -0.69224)
    .height(1380.3);
const POSITION: Float3 = Float3::new(4742.3, 1380.3, 5785.75);

pub fn reset_camera(spring: &spring::Spring<'_, spring::Ui>) {
    if !place_camera(spring, &VIEW, POSITION) {
        spring::log_warning!(
            spring,
            crate::diagnostics::SECTION,
            "the engine refused the camera"
        );
    }
}
