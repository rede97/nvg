use anyhow::Error;
use nvg::*;
use core::f32;
use std::time::Instant;

mod demo;

struct DemoDraw {
    img: Option<ImageId>,
    start_time: Instant,
}

impl<R: Renderer> demo::Demo<R> for DemoDraw {
    fn init(&mut self, ctx: &mut Context<R>) -> Result<(), Error> {
        ctx.create_font_from_file("roboto", "nvg-gl/examples/Roboto-Bold.ttf")?;
        self.img = Some(ctx.create_image_from_file(
            ImageFlags::REPEATX | ImageFlags::REPEATY,
            "nvg-gl/examples/lenna.png",
        )?);
        // unsafe  {
        //     gl::Enable(gl::BLEND);
        //     gl::Enable(gl::POLYGON_SMOOTH);
        //     gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        //     gl::Hint(gl::POLYGON_SMOOTH_HINT, gl::FASTEST);
        // }
        Ok(())
    }

    fn update(&mut self, _width: f32, _height: f32, ctx: &mut Context<R>) -> anyhow::Result<()> {
        // let elapsed = self.start_time.elapsed().as_secs_f32();

        // ctx.begin_path();
        // let radius = 100.0;
        // let distance = 500.0; // Distance to roll
        // let rolled = ((elapsed / 5.0).sin() * 0.5 + 0.5) * distance; // Distance currently rolled
        // let origin = (rolled + 100.0, 600.0);
        // ctx.fill_paint({
        //     ImagePattern {
        //         img: self.img.unwrap(),
        //         center: origin.into(),
        //         size: (100.0, 100.0).into(),
        //         angle: rolled / (2.0 * PI * radius) * 2.0 * PI,
        //         alpha: 1.0,
        //     }
        // });
        // ctx.scissor((150, 600, 1000, 200));
        // ctx.circle(origin, radius);
        // ctx.fill()?;

        // ctx.reset_scissor();
        ctx.save();
        ctx.rotate(f32::consts::PI / 6.0);
        ctx.begin_path();
        ctx.move_to((200.0, 200.0));
        ctx.line_to((600.0, 200.0));
        ctx.line_to((400.0, 100.0));
        ctx.line_to((400.0, 600.0));
        ctx.close_path();
        ctx.restore();
        ctx.circle((700, 500), 400.0);
        // ctx.rounded_rect((200, 200, 300, 300), 20.0);
        // ctx.circle((300.0, 300.0), 30.0);
        // ctx.fill_paint(Color::rgba(0.2, 0.2, 0.6, 1.0));
        // ctx.fill()?;
        // ctx.line_join(LineJoin::Round);
        // ctx.path_solidity(Solidity::Hole);
        // ctx.fill_paint(Color::rgba(0., 0.8, 0.8, 1.0));
        // ctx.fill()?;
        // ctx.close_path();
        ctx.stroke_paint(Color::rgba(1.0, 1.0, 1.0, 1.0));
        ctx.stroke_width(4.0); 
        ctx.stroke()?;

        Ok(())
    }
}

fn main() {
    demo::run(
        DemoDraw {
            img: None,
            start_time: Instant::now(),
        },
        "demo-draw",
    );
}
