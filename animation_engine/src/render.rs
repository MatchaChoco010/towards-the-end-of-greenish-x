// use crate::components::*;
use crate::animation_components::*;
use ggez::*;
use legion::*;

pub(crate) fn render(ctx: &mut Context, world: &World) -> GameResult<()> {
    graphics::clear(ctx, graphics::Color::WHITE);

    let mut renderable_data = <(&Renderable, &Position, Option<&Color>)>::query()
        .iter(world)
        .collect::<Vec<_>>();
    renderable_data.sort_by(|(_, p0, _), (_, p1, _)| p0.z.cmp(&p1.z));

    let draw_param = graphics::DrawParam::default();
    for (renderable, pos, color) in renderable_data {
        match renderable {
            Renderable::Rect { width, height } => {
                let &Color { r, g, b } = color.unwrap_or(&Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                });
                let color = graphics::Color::new(r, g, b, 1.0);
                let mesh = &graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::Fill(graphics::FillOptions::default()),
                    graphics::Rect::new(pos.x, pos.y, *width, *height),
                    color,
                )?;
                graphics::draw(ctx, mesh, draw_param)?
            }
        }
    }

    graphics::present(ctx)
}
