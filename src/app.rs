use crate::node::Node;
use eframe::egui;

/// Configuration and runner for a Topia desktop window.
#[derive(Debug, Clone)]
pub struct App {
    /// Window title displayed in the OS title bar.
    pub title: String,
    /// Logical width of the window.
    pub width: f32,
    /// Logical height of the window.
    pub height: f32,
    /// Whether the window is user-resizable.
    pub resizable: bool,
}

impl App {
    /// Creates a new App configuration with default resizable = true.
    pub fn new(title: impl Into<String>, width: f32, height: f32) -> Self {
        Self {
            title: title.into(),
            width,
            height,
            resizable: true,
        }
    }

    /// Sets whether the window can be resized by the user.
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Fluent alias for `with_resizable`.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Sets the window width and height.
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Sets the window title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Launches the native desktop GUI event loop using `eframe::run_native`.
    ///
    /// This method blocks the calling thread until the window is closed.
    /// On every frame update, `view_builder` is invoked to construct the declarative
    /// `Node` tree which is then rendered into the UI.
    pub fn run<F>(self, view_builder: F) -> Result<(), String>
    where
        F: FnMut() -> Node + 'static,
    {
        let title = self.title.clone();
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([self.width, self.height])
                .with_title(&title)
                .with_resizable(self.resizable),
            ..Default::default()
        };

        eframe::run_native(
            &title,
            native_options,
            Box::new(move |_cc| {
                Ok(Box::new(TopiaApp::new(view_builder)))
            }),
        )
        .map_err(|e| e.to_string())
    }
}

/// The `eframe::App` implementation for Topia.
///
/// Encapsulates the dynamic view builder closure and renders the resulting
/// `Node` tree during every frame tick.
pub struct TopiaApp {
    pub view_builder: Box<dyn FnMut() -> Node + 'static>,
}

impl TopiaApp {
    /// Creates a new `TopiaApp` instance wrapping the given view builder closure.
    pub fn new<F>(view_builder: F) -> Self
    where
        F: FnMut() -> Node + 'static,
    {
        Self {
            view_builder: Box::new(view_builder),
        }
    }

    /// Evaluates the view builder closure and renders the declarative tree into `ui`.
    /// Exposed for both native frame rendering and headless unit test evaluation.
    pub fn render_frame(&mut self, ui: &mut egui::Ui) {
        let mut root = (self.view_builder)();
        root.render(ui);
    }
}

impl eframe::App for TopiaApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            self.render_frame(ui);
        });
    }
}

/// Alias for TopiaApp matching eframe naming conventions.
pub type TopiaEframeApp = TopiaApp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new() {
        let app = App::new("Counter App", 400.0, 300.0);
        assert_eq!(app.title, "Counter App");
        assert_eq!(app.width, 400.0);
        assert_eq!(app.height, 300.0);
        assert!(app.resizable);
    }

    #[test]
    fn test_app_builder_methods() {
        let app = App::new("Initial", 100.0, 100.0)
            .with_title("Custom")
            .with_size(800.0, 600.0)
            .with_resizable(false);

        assert_eq!(app.title, "Custom");
        assert_eq!(app.width, 800.0);
        assert_eq!(app.height, 600.0);
        assert!(!app.resizable);

        let app2 = app.resizable(true);
        assert!(app2.resizable);
    }

    #[test]
    fn test_topia_app_view_builder_evaluation() {
        let mut frame_count = 0;
        let mut app = TopiaApp::new(move || {
            frame_count += 1;
            Node::text(format!("Frame {}", frame_count))
        });

        let node1 = (app.view_builder)();
        assert_eq!(node1.as_text(), Some("Frame 1"));

        let node2 = (app.view_builder)();
        assert_eq!(node2.as_text(), Some("Frame 2"));
    }
}
