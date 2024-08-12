use anyhow::Error;
use nvg::*;
use core::f32;
use std::f32::consts::PI;
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
        Ok(())
    }

    fn update(&mut self, _width: f32, _height: f32, ctx: &mut Context<R>) -> anyhow::Result<()> {
        let elapsed = self.start_time.elapsed().as_secs_f32();

        ctx.begin_path();
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

        ctx.rotate(f32::consts::PI / 6.0);
        ctx.begin_path();
        ctx.move_to((200.0, 200.0));
        ctx.line_to((600.0, 200.0));
        ctx.line_to((400.0, 100.0));
        ctx.line_to((400.0, 600.0));
        ctx.close_path();
        // let color = Color::lerp(
        //     Color::rgb_i(0x2e, 0x50, 0x77),
        //     Color::rgb_i(0xff, 0xca, 0x77),
        //     elapsed.sin() * 0.5 + 0.5,
        // );
        ctx.fill_paint(Color::rgba(0.2, 0.6, 0.2, 1.0));
        ctx.fill()?;
        // ctx.stroke_paint(Color::rgba(0.2, 0.6, 0.2, 1.0));
        // ctx.stroke_width(2.0); 
        // ctx.stroke()?;

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
