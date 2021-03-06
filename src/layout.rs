use std::fmt;

use crate::stack::Stack;
use crate::x::{Connection, WindowGeometry, WindowId};
use crate::Viewport;

mod cmaster;
mod tile;

pub use self::cmaster::CenterMaster;
pub use self::tile::TileLayout;

pub trait LayoutClone {
    fn clone_box(&self) -> Box<dyn Layout>;
}

impl<T> LayoutClone for T
where
    T: 'static + Layout + Clone,
{
    fn clone_box(&self) -> Box<dyn Layout> {
        Box::new(self.clone())
    }
}

pub trait Layout: LayoutClone {
    fn name(&self) -> &str;
    fn layout(
        &self,
        connection: &Connection,
        viewport: &Viewport,
        stack: &Stack<WindowId>,
        master: &Option<WindowId>,
    );
    fn decrease_master(&mut self, viewport: &Viewport, resize_amount: i16);
    fn increase_master(&mut self, viewport: &Viewport, resize_amount: i16);
    fn increase_innergaps(&mut self);
    fn decrease_innergaps(&mut self);
    fn decrease_outergaps(&mut self);
    fn increase_outergaps(&mut self);
}

impl Clone for Box<dyn Layout> {
    fn clone(&self) -> Box<dyn Layout> {
        self.clone_box()
    }
}

impl fmt::Debug for dyn Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Layout {{ \"{}\" }}", self.name())
    }
}

fn configure_single_window(connection: &Connection, viewport: &Viewport, window_id: &WindowId) {
    connection.disable_window_tracking(window_id);
    connection.map_window(window_id);
    connection.configure_window(window_id, &WindowGeometry::default(viewport));
    connection.enable_window_tracking(window_id);
}
