use crate::animation_components::*;
use crate::font_store::*;
use crate::image_store::*;
use crate::localize::*;
use ggez::*;
use legion::*;

pub(crate) fn render(ctx: &mut Context, world: &World, resources: &Resources) -> GameResult {
    let clear_color = *resources.get::<graphics::Color>().unwrap();
    graphics::clear(ctx, clear_color);

    let image_store = resources.get::<ImageStore>().unwrap();
    let font_store = resources.get::<FontStore>().unwrap();

    let mut renderable_data = <(
        &Position,
        Option<&UniformScale>,
        Option<&Rotation>,
        Option<&Color>,
        Option<&Opacity>,
        &Renderable,
    )>::query()
    .iter(world)
    .collect::<Vec<_>>();
    renderable_data.sort_by(|t0, t1| t0.0.z.cmp(&t1.0.z));

    for (pos, scale, rotation, color, opacity, renderable) in renderable_data {
        let &UniformScale { scale } = scale.unwrap_or(&UniformScale { scale: 1.0 });
        let &Rotation { rotation } = rotation.unwrap_or(&Rotation { rotation: 0.0 });
        let &Color { r, g, b } = color.unwrap_or(&Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        });
        let &Opacity { opacity } = opacity.unwrap_or(&Opacity { opacity: 1.0 });

        match renderable {
            Renderable::Rect { width, height } => {
                let color = graphics::Color::new(r, g, b, opacity);
                let mesh = &graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::Fill(graphics::FillOptions::default()),
                    graphics::Rect::new(pos.x, pos.y, *width, *height),
                    color,
                )?;
                let draw_param = graphics::DrawParam::new()
                    .scale(mint::Vector2 { x: scale, y: scale })
                    .rotation(rotation);
                graphics::draw(ctx, mesh, draw_param)?
            }
            Renderable::Image { image: uuid } => {
                if uuid == &uuid::Uuid::nil() {
                    continue;
                }
                let image = image_store.get_image(&uuid).expect("Failed to get image");
                let color = graphics::Color::new(r, g, b, opacity);
                let draw_param = graphics::DrawParam::new()
                    .dest(mint::Point2 { x: pos.x, y: pos.y })
                    .scale(mint::Vector2 { x: scale, y: scale })
                    .rotation(rotation)
                    .color(color);
                graphics::draw(ctx, image, draw_param)?;
            }
            Renderable::Text {
                key,
                font_size,
                format_args,
            } => {
                if key == "" {
                    continue;
                }
                let localize = resources
                    .get::<Box<dyn Localize>>()
                    .expect("Not set localize object!");
                let LocalizeText { font_name, text } = localize.get(key);
                let mut text = text.to_string();
                for arg in format_args {
                    text = text.replacen("{}", arg, 1);
                }
                let mut text = graphics::Text::new(text);
                let font = font_store.get_font(font_name)?;
                text.set_font(font.to_owned(), graphics::PxScale::from(*font_size));
                let color = graphics::Color::new(r, g, b, opacity);
                let draw_param = graphics::DrawParam::new()
                    .dest(mint::Point2 { x: pos.x, y: pos.y })
                    .scale(mint::Vector2 { x: scale, y: scale })
                    .rotation(rotation)
                    .color(color);
                graphics::draw(ctx, &text, draw_param)?;
            }
        }
    }

    graphics::present(ctx)
}
