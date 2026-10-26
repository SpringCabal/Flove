use spring::Float3;
use springcabal_control::camera::{View, place_camera};

const VIEW: View = View::spring(2_384.895, 2.376_999)
    .pitch(-0.721_663_6, -0.692_243_9)
    .height(1_380.307_4);
const POSITION: Float3 = Float3::new(4_742.300_3, 1_380.307_4, 5_785.756);

pub fn reset_camera(spring: &spring::Spring<'_, spring::Ui>) {
    if !place_camera(spring, &VIEW, POSITION) {
        spring::log_warning!(
            spring,
            crate::diagnostics::SECTION,
            "the engine refused the camera"
        );
    }
}
