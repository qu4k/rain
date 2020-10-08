use image::RgbaImage;
use piston_window::EventLoop;

pub fn display_image(image: &RgbaImage) {
  let mut window: piston_window::PistonWindow =
    piston_window::WindowSettings::new(
      "Raytracer",
      [image.width(), image.height()],
    )
    .resizable(false)
    .exit_on_esc(true)
    .build()
    .unwrap_or_else(|_e| panic!("Could not create window!"));

  let tex = piston_window::Texture::from_image(
    &mut window.create_texture_context(),
    &image,
    &piston_window::TextureSettings::new(),
  )
  .unwrap();

  window.set_lazy(true);

  while let Some(e) = window.next() {
    window.draw_2d(&e, |c, g, _| {
      piston_window::clear([1.0; 4], g);
      piston_window::image(&tex, c.transform, g);
    });
  }
}
