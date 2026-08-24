use eframe::egui;

/// Closure callback type for interactive elements such as buttons.
pub type Callback = Box<dyn FnMut() + 'static>;

/// Declarative UI tree node representing a visual component or container layout.
pub enum Node {
    /// Text display component.
    Text {
        text: String,
    },
    /// Interactive button component with a click callback.
    Button {
        label: String,
        on_click: Callback,
    },
    /// Vertical container laying out children from top to bottom.
    VStack {
        children: Vec<Node>,
        spacing: Option<f32>,
    },
    /// Horizontal container laying out children from left to right.
    HStack {
        children: Vec<Node>,
        spacing: Option<f32>,
    },
    /// Empty placeholder node that produces no visual output.
    Empty,
}

impl Node {
    /// Creates a new `Text` node.
    pub fn text(text: impl Into<String>) -> Self {
        Node::Text {
            text: text.into(),
        }
    }

    /// Creates a new `Button` node with a label and click callback closure.
    pub fn button<F>(label: impl Into<String>, on_click: F) -> Self
    where
        F: FnMut() + 'static,
    {
        Node::Button {
            label: label.into(),
            on_click: Box::new(on_click),
        }
    }

    /// Creates a new `VStack` layout with default item spacing.
    pub fn vstack(children: Vec<Node>) -> Self {
        Node::VStack {
            children,
            spacing: None,
        }
    }

    /// Creates a new `VStack` layout with explicit vertical item spacing.
    pub fn vstack_with_spacing(children: Vec<Node>, spacing: f32) -> Self {
        Node::VStack {
            children,
            spacing: Some(spacing),
        }
    }

    /// Creates a new `HStack` layout with default item spacing.
    pub fn hstack(children: Vec<Node>) -> Self {
        Node::HStack {
            children,
            spacing: None,
        }
    }

    /// Creates a new `HStack` layout with explicit horizontal item spacing.
    pub fn hstack_with_spacing(children: Vec<Node>, spacing: f32) -> Self {
        Node::HStack {
            children,
            spacing: Some(spacing),
        }
    }

    /// Creates an `Empty` node.
    pub fn empty() -> Self {
        Node::Empty
    }

    /// Returns `true` if this is an `Empty` node.
    pub fn is_empty(&self) -> bool {
        matches!(self, Node::Empty)
    }

    /// Returns the text content if this is a `Text` node.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Node::Text { text } => Some(text.as_str()),
            _ => None,
        }
    }

    /// Returns the text content if this is a `Text` node (alias for `as_text`).
    pub fn text_content(&self) -> Option<&str> {
        self.as_text()
    }

    /// Returns the button label if this is a `Button` node.
    pub fn as_button_label(&self) -> Option<&str> {
        match self {
            Node::Button { label, .. } => Some(label.as_str()),
            _ => None,
        }
    }

    /// Returns the button label if this is a `Button` node (alias for `as_button_label`).
    pub fn button_label(&self) -> Option<&str> {
        self.as_button_label()
    }

    /// Returns a slice of children if this is a `VStack` or `HStack`, or empty slice otherwise.
    pub fn children(&self) -> &[Node] {
        match self {
            Node::VStack { children, .. } | Node::HStack { children, .. } => children.as_slice(),
            _ => &[],
        }
    }

    /// Returns the number of direct child nodes.
    pub fn child_count(&self) -> usize {
        self.children().len()
    }

    /// Manually fires the button's click callback (if this node is a `Button`).
    /// Returns `true` if the node was a `Button` and the callback was executed; `false` otherwise.
    pub fn fire_click(&mut self) -> bool {
        if let Node::Button { on_click, .. } = self {
            (on_click)();
            true
        } else {
            false
        }
    }

    /// Alias for `fire_click`.
    pub fn trigger_click(&mut self) -> bool {
        self.fire_click()
    }

    /// Traverse the node tree and render components into the provided `egui::Ui`.
    pub fn render(&mut self, ui: &mut egui::Ui) {
        match self {
            Node::Text { text } => {
                ui.label(text.as_str());
            }
            Node::Button { label, on_click } => {
                if ui.button(label.as_str()).clicked() {
                    (on_click)();
                }
            }
            Node::VStack { children, spacing } => {
                ui.vertical(|ui| {
                    if let Some(s) = spacing {
                        ui.spacing_mut().item_spacing.y = *s;
                    }
                    for child in children {
                        child.render(ui);
                    }
                });
            }
            Node::HStack { children, spacing } => {
                ui.horizontal(|ui| {
                    if let Some(s) = spacing {
                        ui.spacing_mut().item_spacing.x = *s;
                    }
                    for child in children {
                        child.render(ui);
                    }
                });
            }
            Node::Empty => {}
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node::Empty
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Text { text } => f.debug_struct("Text").field("text", text).finish(),
            Node::Button { label, .. } => f
                .debug_struct("Button")
                .field("label", label)
                .field("on_click", &"<closure>")
                .finish(),
            Node::VStack { children, spacing } => f
                .debug_struct("VStack")
                .field("children", children)
                .field("spacing", spacing)
                .finish(),
            Node::HStack { children, spacing } => f
                .debug_struct("HStack")
                .field("children", children)
                .field("spacing", spacing)
                .finish(),
            Node::Empty => write!(f, "Empty"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_node_constructors_and_inspectors() {
        let text_node = Node::text("Hello Topia");
        assert_eq!(text_node.text_content(), Some("Hello Topia"));
        assert_eq!(text_node.as_text(), Some("Hello Topia"));
        assert_eq!(text_node.child_count(), 0);

        let button_node = Node::button("Click Me", || {});
        assert_eq!(button_node.button_label(), Some("Click Me"));
        assert_eq!(button_node.as_button_label(), Some("Click Me"));
        assert_eq!(button_node.child_count(), 0);

        let empty_node = Node::empty();
        assert!(empty_node.is_empty());
        assert_eq!(empty_node.child_count(), 0);
        assert_eq!(empty_node.as_text(), None);
        assert_eq!(empty_node.as_button_label(), None);

        let vstack = Node::vstack(vec![
            Node::text("Child 1"),
            Node::text("Child 2"),
        ]);
        assert_eq!(vstack.child_count(), 2);
        assert_eq!(vstack.children().len(), 2);

        let hstack = Node::hstack_with_spacing(
            vec![Node::button("B1", || {}), Node::button("B2", || {})],
            10.0,
        );
        assert_eq!(hstack.child_count(), 2);
    }

    #[test]
    fn test_button_trigger_click() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        let mut button = Node::button("Increment", move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(counter.load(Ordering::SeqCst), 0);
        let executed = button.trigger_click();
        assert!(executed);
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        button.fire_click();
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_non_button_fire_click_returns_false() {
        let mut text = Node::text("Static");
        assert!(!text.fire_click());

        let mut empty = Node::empty();
        assert!(!empty.fire_click());
    }

    #[test]
    fn test_debug_formatting() {
        let tree = Node::vstack(vec![
            Node::text("Header"),
            Node::button("Submit", || {}),
            Node::Empty,
        ]);
        let debug_str = format!("{:?}", tree);
        assert!(debug_str.contains("VStack"));
        assert!(debug_str.contains("Header"));
        assert!(debug_str.contains("Submit"));
        assert!(debug_str.contains("<closure>"));
        assert!(debug_str.contains("Empty"));
    }

    #[test]
    fn test_default_impl() {
        let default_node: Node = Default::default();
        assert!(default_node.is_empty());
    }
}
