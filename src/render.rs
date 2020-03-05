//! Rendering is RLBot's ability to draw directly inside the game window.

use crate::{flat, rlbot::RLBot};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::error::Error;

/// A render group in the process of being built.
///
/// The drawing methods in this class simply collect a list of items to be
/// drawn later. Nothing will actually be drawn to screen until
/// [`render`](RenderGroup::render) is called.
///
/// # Examples
///
/// ## Basic rendering
///
/// ```no_run
/// # use rlbot::flat::ColorArgs;
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<Error>> {
/// let rlbot = rlbot::init()?;
/// let mut group = rlbot.begin_render_group(1234);
/// let green = group.color_rgb(0, 255, 0);
/// group.draw_string_2d((10.0, 10.0), (2, 2), "I am text!", green);
/// group.render()?;
/// # Ok(())
/// # }
/// ```
///
/// ## Clearing the screen
///
/// ```no_run
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<Error>> {
/// let rlbot = rlbot::init()?;
/// let group = rlbot.begin_render_group(1234);
/// group.render()?;
/// # Ok(())
/// # }
/// ```
pub struct RenderGroup<'a> {
    rlbot: &'a RLBot,
    id: i32,
    builder: FlatBufferBuilder<'a>,
    messages: Vec<WIPOffset<flat::RenderMessage<'a>>>,
}

impl<'a> RenderGroup<'a> {
    pub(crate) fn new(rlbot: &'a RLBot, id: i32) -> Self {
        Self {
            rlbot,
            id,
            builder: FlatBufferBuilder::new_with_capacity(1024),
            messages: Vec::new(),
        }
    }
}

/// A color that can be used to draw in a [`RenderGroup`].
#[derive(Copy, Clone)]
pub struct Color<'a>(WIPOffset<flat::Color<'a>>);

impl<'a> RenderGroup<'a> {
    /// Send the collected drawings to RLBot to be rendered to screen.
    pub fn render(mut self) -> Result<(), Box<dyn Error>> {
        let messages = self.builder.create_vector(&self.messages);

        let render_group = {
            let mut rg = flat::RenderGroupBuilder::new(&mut self.builder);
            rg.add_renderMessages(messages);
            rg.add_id(self.id);
            rg.finish()
        };

        self.builder.finish(render_group, None);
        let data = self.builder.finished_data();
        self.rlbot.interface().render_group(data)?;
        Ok(())
    }

    /// Create a color with the given **a**lpha, **r**ed, **g**reen, and
    /// **b**lue. An alpha of 255 is fully opaque, and 0 is fully transparent.
    ///
    /// A color can only be used with the `RenderGroup` that created it.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rlbot::RenderGroup;
    /// # let mut group: RenderGroup = unsafe { ::std::mem::uninitialized() };
    /// let transbluecent = group.color_argb(127, 0, 0, 255);
    /// ```
    pub fn color_argb(&mut self, a: u8, r: u8, g: u8, b: u8) -> Color<'a> {
        let args = flat::ColorArgs { a, r, g, b };
        Color(flat::Color::create(&mut self.builder, &args))
    }

    /// Create an opaque color with the given, **r**ed, **g**reen, and **b**lue.
    ///
    /// A color can only be used with the `RenderGroup` that created it.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rlbot::RenderGroup;
    /// # let mut group: RenderGroup = unsafe { ::std::mem::uninitialized() };
    /// let green = group.color_rgb(0, 255, 0);
    /// ```
    pub fn color_rgb(&mut self, r: u8, g: u8, b: u8) -> Color<'a> {
        let args = flat::ColorArgs { a: 255, r, g, b };
        Color(flat::Color::create(&mut self.builder, &args))
    }

    /// Draw a line using screen coordinates.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rlbot::RenderGroup;
    /// # let mut group: RenderGroup = unsafe { ::std::mem::uninitialized() };
    /// # let green = group.color_rgb(0, 255, 0);
    /// group.draw_line_2d((10.0, 10.0), (100.0, 100.0), green);
    /// ```
    #[deprecated(note = "Drawing 2D lines is not currently supported")]
    pub fn draw_line_2d(
        &mut self,
        (x1, y1): (f32, f32),
        (x2, y2): (f32, f32),
        Color(color): Color<'_>,
    ) {
        let start = flat::Vector3::new(x1, y1, 0.0);
        let end = flat::Vector3::new(x2, y2, 0.0);

        let mut rm = flat::RenderMessageBuilder::new(&mut self.builder);
        rm.add_renderType(flat::RenderType::DrawLine2D);
        rm.add_color(color);
        rm.add_start(&start);
        rm.add_end(&end);
        self.messages.push(rm.finish());
    }

    /// Draw a line using world coordinates.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rlbot::RenderGroup;
    /// # let mut group: RenderGroup = unsafe { ::std::mem::uninitialized() };
    /// # let green = group.color_rgb(0, 255, 0);
    /// group.draw_line_3d((10.0, 10.0, 10.0), (100.0, 100.0, 100.0), green);
    /// ```
    pub fn draw_line_3d(
        &mut self,
        (x1, y1, z1): (f32, f32, f32),
        (x2, y2, z2): (f32, f32, f32),
        Color(color): Color<'_>,
    ) {
        let start = flat::Vector3::new(x1, y1, z1);
        let end = flat::Vector3::new(x2, y2, z2);

        let mut rm = flat::RenderMessageBuilder::new(&mut self.builder);
        rm.add_renderType(flat::RenderType::DrawLine3D);
        rm.add_color(color);
        rm.add_start(&start);
        rm.add_end(&end);
        self.messages.push(rm.finish());
    }

    /// Draw a line with one endpoint in screen coordinates and the other at a
    /// point projected from world coordinates to screen coordinates.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rlbot::RenderGroup;
    /// # let mut group: RenderGroup = unsafe { ::std::mem::uninitialized() };
    /// # let green = group.color_rgb(0, 255, 0);
    /// group.draw_line_2d_3d((10.0, 10.0), (100.0, 100.0, 100.0), green);
    /// ```
    #[deprecated(note = "Drawing 2D-3D lines is not currently supported")]
    pub fn draw_line_2d_3d(
        &mut self,
        (x1, y1): (f32, f32),
        (x2, y2, z2): (f32, f32, f32),
        Color(color): Color<'_>,
    ) {
        let start = flat::Vector3::new(x1, y1, 0.0);
        let end = flat::Vector3::new(x2, y2, z2);

        let mut rm = flat::RenderMessageBuilder::new(&mut self.builder);
        rm.add_renderType(flat::RenderType::DrawLine2D_3D);
        rm.add_color(color);
        rm.add_start(&start);
        rm.add_end(&end);
        self.messages.push(rm.finish());
    }

    /// Draw text using screen coordinates.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rlbot::RenderGroup;
    /// # let mut group: RenderGroup = unsafe { ::std::mem::uninitialized() };
    /// # let green = group.color_rgb(0, 255, 0);
    /// group.draw_string_2d((10.0, 10.0), (2, 2), "I am text!", green);
    /// ```
    pub fn draw_string_2d(
        &mut self,
        (x, y): (f32, f32),
        (scale_x, scale_y): (i32, i32),
        text: impl AsRef<str>,
        Color(color): Color<'_>,
    ) {
        let start = flat::Vector3::new(x, y, 0.0);
        let text = self.builder.create_string(text.as_ref());

        let mut rm = flat::RenderMessageBuilder::new(&mut self.builder);
        rm.add_renderType(flat::RenderType::DrawString2D);
        rm.add_color(color);
        rm.add_start(&start);
        rm.add_scaleX(scale_x);
        rm.add_scaleY(scale_y);
        rm.add_text(text);
        self.messages.push(rm.finish());
    }

    /// Draw text at a point projected from world coordinates to screen
    /// coordinates.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rlbot::RenderGroup;
    /// # let mut group: RenderGroup = unsafe { ::std::mem::uninitialized() };
    /// # let green = group.color_rgb(0, 255, 0);
    /// group.draw_string_3d((10.0, 10.0, 10.0), (2, 2), "I am text!", green);
    /// ```
    pub fn draw_string_3d(
        &mut self,
        (x, y, z): (f32, f32, f32),
        (scale_x, scale_y): (i32, i32),
        text: impl AsRef<str>,
        Color(color): Color<'_>,
    ) {
        let start = flat::Vector3::new(x, y, z);
        let text = self.builder.create_string(text.as_ref());

        let mut rm = flat::RenderMessageBuilder::new(&mut self.builder);
        rm.add_renderType(flat::RenderType::DrawString3D);
        rm.add_color(color);
        rm.add_start(&start);
        rm.add_scaleX(scale_x);
        rm.add_scaleY(scale_y);
        rm.add_text(text);
        self.messages.push(rm.finish());
    }
}
