import re

with open("src/node.rs", "r") as f:
    content = f.read()

# Update the Node enum
new_enum = """pub enum Node {
    /// Text display component.
    Text {
        text: String,
        size: Option<f32>,
        bold: bool,
    },
    /// Interactive text input component.
    TextInput {
        text: String,
        on_change: Box<dyn FnMut(String) + 'static>,
    },
    /// Interactive checkbox component.
    Checkbox {
        checked: bool,
        label: String,
        on_change: Box<dyn FnMut(bool) + 'static>,
    },
    /// Interactive button component with a click callback.
    Button {"""

content = re.sub(r'pub enum Node \{\s*/// Text display component\.\s*Text \{\s*text: String,\s*\},.*?Button \{', new_enum, content, flags=re.DOTALL)

# Update Node::text constructor
new_text_fn = """pub fn text(text: impl Into<String>) -> Self {
        Node::Text {
            text: text.into(),
            size: None,
            bold: false,
        }
    }

    /// Creates a new `Text` node with styling.
    pub fn text_styled(text: impl Into<String>, size: Option<f32>, bold: bool) -> Self {
        Node::Text {
            text: text.into(),
            size,
            bold,
        }
    }

    /// Creates a new `TextInput` node.
    pub fn text_input<F>(text: impl Into<String>, on_change: F) -> Self
    where
        F: FnMut(String) + 'static,
    {
        Node::TextInput {
            text: text.into(),
            on_change: Box::new(on_change),
        }
    }

    /// Creates a new `Checkbox` node.
    pub fn checkbox<F>(checked: bool, label: impl Into<String>, on_change: F) -> Self
    where
        F: FnMut(bool) + 'static,
    {
        Node::Checkbox {
            checked,
            label: label.into(),
            on_change: Box::new(on_change),
        }
    }

    /// Creates a new `Button`"""

content = re.sub(r'pub fn text\(text: impl Into<String>\) -> Self \{.*?pub fn button', new_text_fn, content, flags=re.DOTALL)

# Update as_text
new_as_text = """Node::Text { text, .. } => Some(text.as_str()),"""
content = re.sub(r'Node::Text \{ text \} => Some\(text\.as_str\(\)\),', new_as_text, content)

# Update format Debug
new_debug = """Node::Text { text, .. } => f.debug_struct("Text").field("text", text).finish(),
            Node::TextInput { text, .. } => f.debug_struct("TextInput").field("text", text).finish(),
            Node::Checkbox { label, checked, .. } => f.debug_struct("Checkbox").field("label", label).field("checked", checked).finish(),
            Node::Button"""
content = re.sub(r'Node::Text \{ text \} => f\.debug_struct\("Text"\)\.field\("text", text\)\.finish\(\),\s*Node::Button', new_debug, content)

# Update render
new_render = """pub fn render(&mut self, ui: &mut egui::Ui) {
        match self {
            Node::Text { text, size, bold } => {
                let mut rt = egui::RichText::new(text.as_str());
                if let Some(s) = size {
                    rt = rt.size(*s);
                }
                if *bold {
                    rt = rt.strong();
                }
                ui.label(rt);
            }
            Node::TextInput { text, on_change } => {
                let mut current_text = text.clone();
                if ui.text_edit_singleline(&mut current_text).changed() {
                    (on_change)(current_text);
                }
            }
            Node::Checkbox { checked, label, on_change } => {
                let mut current = *checked;
                if ui.checkbox(&mut current, label.as_str()).changed() {
                    (on_change)(current);
                }
            }
            Node::Button"""

content = re.sub(r'pub fn render\(&mut self, ui: &mut egui::Ui\) \{\s*match self \{\s*Node::Text \{ text \} => \{\s*ui\.label\(text\.as_str\(\)\);\s*\}\s*Node::Button', new_render, content, flags=re.DOTALL)

with open("src/node.rs", "w") as f:
    f.write(content)
