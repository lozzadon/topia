import re

with open("src/node.rs", "r") as f:
    content = f.read()

# 1. Update Node enum
# find Text { text: String } and replace it
content = content.replace("Text {\n        text: String,\n    },", "Text {\n        text: String,\n        size: Option<f32>,\n        bold: bool,\n    },\n    TextInput {\n        text: String,\n        on_change: Box<dyn FnMut(String) + 'static>,\n    },\n    Checkbox {\n        checked: bool,\n        label: String,\n        on_change: Box<dyn FnMut(bool) + 'static>,\n    },")

# 2. Update text constructor
content = content.replace("pub fn text(text: impl Into<String>) -> Self {\n        Node::Text {\n            text: text.into(),\n        }\n    }", "pub fn text(text: impl Into<String>) -> Self {\n        Node::Text {\n            text: text.into(),\n            size: None,\n            bold: false,\n        }\n    }\n\n    pub fn text_styled(text: impl Into<String>, size: Option<f32>, bold: bool) -> Self {\n        Node::Text {\n            text: text.into(),\n            size,\n            bold,\n        }\n    }\n\n    pub fn text_input<F>(text: impl Into<String>, on_change: F) -> Self\n    where\n        F: FnMut(String) + 'static,\n    {\n        Node::TextInput {\n            text: text.into(),\n            on_change: Box::new(on_change),\n        }\n    }\n\n    pub fn checkbox<F>(checked: bool, label: impl Into<String>, on_change: F) -> Self\n    where\n        F: FnMut(bool) + 'static,\n    {\n        Node::Checkbox {\n            checked,\n            label: label.into(),\n            on_change: Box::new(on_change),\n        }\n    }")

# 3. Update as_text
content = content.replace("Node::Text { text } => Some(text.as_str()),", "Node::Text { text, .. } => Some(text.as_str()),")

# 4. Update render match block
render_match = """Node::Text { text } => {
                ui.label(text.as_str());
            }"""
new_render_match = """Node::Text { text, size, bold } => {
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
            }"""
content = content.replace(render_match, new_render_match)

# 5. Update debug formatting
debug_match = """Node::Text { text } => f.debug_struct("Text").field("text", text).finish(),"""
new_debug_match = """Node::Text { text, .. } => f.debug_struct("Text").field("text", text).finish(),
            Node::TextInput { text, .. } => f.debug_struct("TextInput").field("text", text).finish(),
            Node::Checkbox { label, checked, .. } => f.debug_struct("Checkbox").field("label", label).field("checked", checked).finish(),"""
content = content.replace(debug_match, new_debug_match)

with open("src/node.rs", "w") as f:
    f.write(content)
