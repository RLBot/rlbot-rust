//! Rendering is RLBot's ability to draw directly inside the game window.

use flat;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use rlbot::RLBot;
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
/// group.draw_string_2d(10.0, 10.0, 2, 2, "I am text!", &ColorArgs::rgb(0, 255, 0));
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
    pub(crate) fn new(rlbot: &'a RLBot, id: i32) -> RenderGroup<'a> {
        let builder = FlatBufferBuilder::new_with_capacity(1024);
        RenderGroup {
            rlbot,
            id,
            builder,
            messages: Vec::new(),
        }
    }
}

impl<'a> RenderGroup<'a> {
    /// Send the collected drawings to RLBot to be rendered to screen.
    pub fn render(mut self) -> Result<(), Box<Error>> {
        let messages = self.builder.create_vector(&self.messages);

        let render_group = {
            let mut rg = flat::RenderGroupBuilder::new(&mut self.builder);
            rg.add_renderMessages(messages);
            rg.add_id(self.id);
            rg.finish()
        };

        self.builder.finish(render_group, None);
        let data = self.builder.finished_data();
        self.rlbot.render_group(data)?;
        Ok(())
    }

    /// Draw a line using 2D coordinates.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rlbot::flat::ColorArgs;
    /// # use rlbot::RenderGroup;
    /// # let mut group: RenderGroup = unsafe { ::std::mem::uninitialized() };
    /// group.draw_line_2d(10.0, 10.0, 100.0, 100.0, &ColorArgs::rgb(0, 255, 0));
    /// ```
    pub fn draw_line_2d(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: &flat::ColorArgs) {
        let color = flat::Color::create(&mut self.builder, &color);
        let start = flat::Vector3::new(x1, y1, 0.0);
        let end = flat::Vector3::new(x2, y2, 0.0);

        let mut rm = flat::RenderMessageBuilder::new(&mut self.builder);
        rm.add_renderType(flat::RenderType::DrawLine2D);
        rm.add_color(color);
        rm.add_start(&start);
        rm.add_end(&end);
        self.messages.push(rm.finish());
    }

    /// Draw a string at a 2D coordinate.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rlbot::flat::ColorArgs;
    /// # use rlbot::RenderGroup;
    /// # let mut group: RenderGroup = unsafe { ::std::mem::uninitialized() };
    /// group.draw_string_2d(10.0, 10.0, 2, 2, "I am text!", &ColorArgs::rgb(0, 255, 0));
    /// ```
    pub fn draw_string_2d(
        &mut self,
        x: f32,
        y: f32,
        scale_x: i32,
        scale_y: i32,
        text: impl AsRef<str>,
        color: &flat::ColorArgs,
    ) {
        let color = flat::Color::create(&mut self.builder, color);
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
}
