pub mod graphics;
pub mod input;
pub mod window;

pub trait Game {
    fn update(&mut self, viewport: Option<&graphics::Viewport>);

    fn draw(&mut self, _gpu: &mut graphics::Gpu) -> graphics::Result<()> {
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _keycode: input::Keycode,
        _keymod: input::Mod,
        _repeat: bool,
    ) {
    }

    fn key_up_event(&mut self, _keycode: input::Keycode, _keymod: input::Mod) {}
}

pub fn run<G: Game>(
    game: &mut G,
    gpu: Option<&mut graphics::Gpu>,
    ticks_per_second: u32,
) -> graphics::Result<()> {
    match gpu {
        Some(gpu) => run_with_gpu(game, gpu, ticks_per_second),
        None => {
            run_headless(game, ticks_per_second);
            Ok(())
        }
    }
}

fn run_with_gpu<G: Game>(
    game: &mut G,
    gpu: &mut graphics::Gpu,
    ticks_per_second: u32,
) -> graphics::Result<()> {
    let mut alive = true;

    while alive {
        {
            let window = gpu.window();

            window.poll_events(|event| match event {
                window::Event::CloseRequested => {
                    alive = false;
                }
            });
        }

        game.update(None);
        game.draw(gpu)?;
    }

    Ok(())
}

fn run_headless<G: Game>(game: &mut G, ticks_per_second: u32) {
    unimplemented! {}
}
