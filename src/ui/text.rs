use crate::*;

#[derive(Component)]
pub(crate) struct PhaseBanner;

pub(crate) fn spawn_pixel_text(
    commands: &mut Commands,
    assets: &SpriteAssets,
    text: &str,
    top_left: Vec2,
    z: f32,
) {
    spawn_pixel_text_inner(commands, assets, text, top_left, z, false);
}

pub(crate) fn spawn_phase_text(
    commands: &mut Commands,
    assets: &SpriteAssets,
    lines: &[String],
    center_y: f32,
    z: f32,
) {
    let line_gap = 3.0;
    let line_height = 7.0;
    let text_step = line_height + line_gap;
    let text_block_height =
        lines.len() as f32 * line_height + lines.len().saturating_sub(1) as f32 * line_gap;
    let background_height = text_block_height + 10.0;
    let background_top = center_y - background_height / 2.0;
    let first_line_top = center_y - text_block_height / 2.0;

    commands.spawn((
        Sprite::from_color(
            Color::srgb_u8(48, 48, 40),
            Vec2::new(132.0 * window_scale(), background_height * window_scale()),
        ),
        Transform::from_translation(virtual_center_scaled(
            Vec2::new(36.0, background_top),
            Vec2::new(132.0, background_height),
            z - 0.1,
        )),
        PhaseBanner,
        GameEntity,
    ));

    for (index, line) in lines.iter().enumerate() {
        let text_width = phase_text_width(line);
        spawn_pixel_text_inner(
            commands,
            assets,
            line,
            Vec2::new(
                (208.0 - text_width) / 2.0,
                first_line_top + index as f32 * text_step,
            ),
            z,
            true,
        );
    }
}

fn spawn_pixel_text_inner(
    commands: &mut Commands,
    assets: &SpriteAssets,
    text: &str,
    top_left: Vec2,
    z: f32,
    phase_banner: bool,
) {
    for (index, ch) in text.chars().enumerate() {
        if ch == ' ' {
            continue;
        }
        let mut entity = commands.spawn((
            Sprite::from_atlas_image(
                assets.glyph_image.clone(),
                TextureAtlas {
                    layout: assets.glyph_layout.clone(),
                    index: glyph_index(ch, &assets.manifest.glyphs),
                },
            ),
            Transform::from_translation(virtual_center_scaled(
                Vec2::new(top_left.x + index as f32 * GLYPH_ADVANCE, top_left.y),
                glyph_size(&assets.manifest.glyphs),
                z,
            ))
            .with_scale(Vec3::splat(window_scale())),
            GameEntity,
        ));

        if phase_banner {
            entity.insert(PhaseBanner);
        }
    }
}
