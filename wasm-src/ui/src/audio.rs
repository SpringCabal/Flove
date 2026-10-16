const NARRATION_VOLUME: f32 = 50.0;

pub fn play_narration(stage: i32) {
    let file = match stage {
        1 => "sounds/narration_amplified/InTimesLongGone.ogg",
        2 => "sounds/narration_amplified/RepelBorders.ogg",
        3 => "sounds/narration_amplified/TheyRejectOurLoveAndKindness.ogg",
        4 => "sounds/narration_amplified/BlastedAnoterWaveOfEnemies.ogg",
        5 => "sounds/narration_amplified/ThisSeemsToBeTheEndOfThemForNow.ogg",
        6 => "sounds/narration_amplified/ItsABeautifulSightIndeed.ogg",
        7 => "sounds/narration_amplified/AVastEnemyArmyApproaches.ogg",
        8 => "sounds/narration_amplified/TheMushroomKingLiesDefeated.ogg",
        _ => return,
    };
    let _ = spring::sound::play_sound_file(
        file,
        NARRATION_VOLUME,
        spring::types::Float3::ZERO,
        spring::types::Float3::ZERO,
        0,
    );
}
