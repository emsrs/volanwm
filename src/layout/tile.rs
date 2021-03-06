use crate::layout::Layout;
use crate::stack::Stack;
use crate::x::{Connection, WindowGeometry, WindowId};
use crate::Viewport;

#[derive(Clone)]
pub struct TileLayout {
    name: String,
    pub resized_width: i16,
    outergaps: u32,
    innergaps: u32,
}

impl Layout for TileLayout {
    fn name(&self) -> &str {
        &self.name
    }
    fn layout(
        &self,
        connection: &Connection,
        viewport: &Viewport,
        stack: &Stack<WindowId>,
        master: &Option<WindowId>,
    ) {
        if stack.is_empty() {
            return;
        }
        let master_id = if master.is_none() {
            stack.focused().unwrap()
        } else {
            master.as_ref().unwrap()
        };
        if stack.len() < 2 {
            super::configure_single_window(connection, viewport, master_id);
        } else {
            self.tile(connection, viewport, stack, master_id);
        }
    }

    fn decrease_master(&mut self, viewport: &Viewport, resize_amount: i16) {
        if self.resized_width > -((viewport.width / 2) as i16 - (viewport.width / 8) as i16) {
            self.resized_width -= resize_amount;
        }
    }

    fn increase_master(&mut self, viewport: &Viewport, resize_amount: i16) {
        if self.resized_width < ((viewport.width / 2) as i16 - (viewport.width / 8) as i16) {
            self.resized_width += resize_amount;
        }
    }

    fn increase_innergaps(&mut self) {
        self.innergaps += 1;
        info!("Increased innergaps to {}", self.innergaps);
    }

    fn decrease_innergaps(&mut self) {
        if (self.innergaps as i32 - 1) >= 0 {
            self.innergaps -= 1;
            info!("Decreased innergaps to {}", self.innergaps);
        } else {
            error!("Further decreasing inner gaps is less than zero");
        }
    }

    fn decrease_outergaps(&mut self) {
        if (self.innergaps as i32 - 1) >= 0 {
            self.innergaps -= 1;
            info!("Decreased outergaps to {}", self.innergaps);
        } else {
            error!("Further decreasing outer gaps is less than zero");
        }
    }

    fn increase_outergaps(&mut self) {
        self.outergaps += 1;
        info!("Increased outergaps to {}", self.outergaps);
    }
}

impl TileLayout {
    pub fn new<S: Into<String>>(name: S, innergaps: u32, outergaps: u32) -> TileLayout {
        Self {
            name: name.into(),
            resized_width: 160,
            innergaps,
            outergaps,
        }
    }

    pub fn tile(
        &self,
        connection: &Connection,
        viewport: &Viewport,
        stack: &Stack<WindowId>,
        master_id: &WindowId,
    ) {
        self.configure_master_window(connection, viewport, master_id);
        let mut accumulator: u32 = 0;
        for window_id in stack.iter() {
            if window_id != master_id {
                self.configure_normal_window(accumulator, connection, stack, viewport, window_id);
                accumulator += 1;
            }
        }
    }

    fn configure_normal_window(
        &self,
        i: u32,
        connection: &Connection,
        stack: &Stack<WindowId>,
        viewport: &Viewport,
        window_id: &WindowId,
    ) {
        let normal_geometry: WindowGeometry = self.get_normal_geometry(i, stack, viewport);
        connection.configure_window(window_id, &normal_geometry);
    }

    fn configure_master_window(
        &self,
        connection: &Connection,
        viewport: &Viewport,
        window_id: &WindowId,
    ) {
        let focused_geometry: WindowGeometry = self.get_master_geometry(viewport);
        connection.configure_window(window_id, &focused_geometry);
    }

    fn get_normal_geometry(
        &self,
        i: u32,
        stack: &Stack<WindowId>,
        viewport: &Viewport,
    ) -> WindowGeometry {
        let x: u32 = ((viewport.width / 2) as i16 + self.resized_width) as u32 + self.innergaps;
        let width: u32 = ((viewport.width / 2) as i16 - self.resized_width) as u32
            - self.outergaps
            - self.innergaps;
        let height: u32 = (viewport.height - self.outergaps * 2 + self.innergaps)
            / (stack.len() - 1) as u32
            - self.innergaps;
        WindowGeometry {
            x,
            y: self.outergaps + (i as u32 * (height + self.innergaps)),
            width,
            height,
        }
    }

    fn get_master_geometry(&self, viewport: &Viewport) -> WindowGeometry {
        let width: u32 =
            ((((viewport.width / 2) as i16) + (self.resized_width)) as u32) - self.outergaps;
        WindowGeometry {
            x: viewport.x + self.outergaps,
            y: viewport.y + self.outergaps,
            width,
            height: viewport.height - self.outergaps * 2,
        }
    }
}
