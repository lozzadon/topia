use eframe::egui;

pub type Callback = Box<dyn FnMut() + 'static>;

pub enum Node {
    Text { text: String, size: Option<f32>, bold: bool },
    TextInput { text: String, on_change: Box<dyn FnMut(String) + 'static> },
    Checkbox { checked: bool, label: String, on_change: Box<dyn FnMut(bool) + 'static> },
    Button { label: String, on_click: Callback },
    VStack { children: Vec<Node>, spacing: Option<f32> },
    HStack { children: Vec<Node>, spacing: Option<f32> },
    Empty,
    Center { child: Box<Node> },
    Scale { scale: f32, child: Box<Node> },
    Slider { value: f32, min: f32, max: f32, on_change: Box<dyn FnMut(f32) + 'static> },
    ScrollArea { children: Vec<Node> },
    Graph { points: Vec<(f32, f32)>, min_x: f32, max_x: f32, min_y: f32, max_y: f32 },
    Separator,
    ProgressBar { progress: f32 },
    Toggle { checked: bool, on_change: Box<dyn FnMut(bool) + 'static> },
    Stepper { value: f32, step: f32, on_change: Box<dyn FnMut(f32) + 'static> },
    ColorWell { color: [u8; 4], on_change: Box<dyn FnMut([u8; 4]) + 'static> },
    ComboBox { selected: String, options: Vec<String>, on_change: Box<dyn FnMut(String) + 'static> },
    SegmentedControl { selected: usize, segments: Vec<String>, on_change: Box<dyn FnMut(usize) + 'static> }
}

impl Node {
    pub fn text<S: Into<String>>(t: S) -> Self { Node::Text { text: t.into(), size: None, bold: false } }
    pub fn text_styled<S: Into<String>>(t: S, size: Option<f32>, bold: bool) -> Self { Node::Text { text: t.into(), size, bold } }
    pub fn button<S: Into<String>, F: FnMut() + 'static>(label: S, cb: F) -> Self { Node::Button { label: label.into(), on_click: Box::new(cb) } }
    pub fn empty() -> Self { Node::Empty }
    pub fn separator() -> Self { Node::Separator }
    pub fn progress_bar(progress: f32) -> Self { Node::ProgressBar { progress } }
    pub fn toggle<F: FnMut(bool) + 'static>(checked: bool, on_change: F) -> Self { Node::Toggle { checked, on_change: Box::new(on_change) } }
    pub fn stepper<F: FnMut(f32) + 'static>(value: f32, step: f32, on_change: F) -> Self { Node::Stepper { value, step, on_change: Box::new(on_change) } }
    pub fn color_well<F: FnMut([u8; 4]) + 'static>(color: [u8; 4], on_change: F) -> Self { Node::ColorWell { color, on_change: Box::new(on_change) } }
    pub fn combo_box<S: Into<String>, F: FnMut(String) + 'static>(selected: S, options: Vec<String>, on_change: F) -> Self { Node::ComboBox { selected: selected.into(), options, on_change: Box::new(on_change) } }
    pub fn segmented_control<F: FnMut(usize) + 'static>(selected: usize, segments: Vec<String>, on_change: F) -> Self { Node::SegmentedControl { selected, segments, on_change: Box::new(on_change) } }

    pub fn vstack(children: Vec<Node>) -> Self { Node::VStack { children, spacing: None } }
    pub fn hstack(children: Vec<Node>) -> Self { Node::HStack { children, spacing: None } }
    
    pub fn text_input<S: Into<String>, F: FnMut(String) + 'static>(text: S, on_change: F) -> Self { Node::TextInput { text: text.into(), on_change: Box::new(on_change) } }
    pub fn checkbox<S: Into<String>, F: FnMut(bool) + 'static>(checked: bool, label: S, on_change: F) -> Self { Node::Checkbox { checked, label: label.into(), on_change: Box::new(on_change) } }
    pub fn vstack_with_spacing(children: Vec<Node>, spacing: f32) -> Self { Node::VStack { children, spacing: Some(spacing) } }
    pub fn hstack_with_spacing(children: Vec<Node>, spacing: f32) -> Self { Node::HStack { children, spacing: Some(spacing) } }
    pub fn center(child: Node) -> Self { Node::Center { child: Box::new(child) } }
    pub fn slider<F: FnMut(f32) + 'static>(value: f32, min: f32, max: f32, on_change: F) -> Self { Node::Slider { value, min, max, on_change: Box::new(on_change) } }
    pub fn scroll_area(children: Vec<Node>) -> Self { Node::ScrollArea { children } }
    pub fn graph(points: Vec<(f32, f32)>, min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Self { Node::Graph { points, min_x, max_x, min_y, max_y } }
    pub fn scale(scale: f32, child: Node) -> Self { Node::Scale { scale, child: Box::new(child) } }

    
    pub fn text_content(&self) -> Option<&str> { if let Node::Text { text, .. } = self { Some(text) } else { None } }
    pub fn as_text(&self) -> Option<&str> { self.text_content() }
    pub fn button_label(&self) -> Option<&str> { if let Node::Button { label, .. } = self { Some(label) } else { None } }
    pub fn as_button_label(&self) -> Option<&str> { self.button_label() }
    pub fn is_empty(&self) -> bool { matches!(self, Node::Empty) }
    pub fn children(&self) -> &[Node] {
        match self {
            Node::VStack { children, .. } | Node::HStack { children, .. } | Node::ScrollArea { children } => children,
            _ => &[],
        }
    }
    pub fn child_count(&self) -> usize { self.children().len() }
    pub fn trigger_click(&mut self) -> bool { self.fire_click() }
    pub fn fire_click(&mut self) -> bool {
        if let Node::Button { on_click, .. } = self {
            on_click();
            true
        } else {
            false
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        match self {
            Node::Text { text, size, bold } => {
                let mut rt = egui::RichText::new(text.as_str());
                if let Some(s) = size { rt = rt.size(*s); }
                if *bold { rt = rt.strong(); }
                ui.label(rt);
            }
            Node::TextInput { text, on_change } => {
                let mut current_text = text.clone();
                if ui.text_edit_singleline(&mut current_text).changed() { (on_change)(current_text); }
            }
            Node::Checkbox { checked, label, on_change } => {
                let mut current = *checked;
                if ui.checkbox(&mut current, label.as_str()).changed() { (on_change)(current); }
            }
            Node::Button { label, on_click } => {
                if ui.button(label.as_str()).clicked() { (on_click)(); }
            }
            Node::VStack { children, spacing } => {
                ui.vertical_centered(|ui| {
                    ui.set_width(ui.available_width());
                    if let Some(s) = spacing { ui.spacing_mut().item_spacing.y = *s; }
                    for (i, child) in children.iter_mut().enumerate() {
                        ui.push_id(i, |ui| { child.render(ui); });
                    }
                });
            }
            Node::HStack { children, spacing } => {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min).with_main_align(egui::Align::Center), |ui| {
                    ui.set_width(ui.available_width());
                    if let Some(s) = spacing { ui.spacing_mut().item_spacing.x = *s; }
                    for (i, child) in children.iter_mut().enumerate() {
                        ui.push_id(i, |ui| { child.render(ui); });
                    }
                });
            }
            Node::Center { child } => {
                ui.vertical_centered_justified(|ui| { child.render(ui); });
            }
            Node::Scale { scale, child } => {
                ui.scope(|ui| {
                    let s = *scale;
                    for (_, font_id) in ui.style_mut().text_styles.iter_mut() { font_id.size *= s; }
                    ui.style_mut().spacing.item_spacing.x *= s;
                    ui.style_mut().spacing.item_spacing.y *= s;
                    ui.style_mut().spacing.button_padding.x *= s;
                    ui.style_mut().spacing.button_padding.y *= s;
                    ui.style_mut().spacing.interact_size.x *= s;
                    ui.style_mut().spacing.interact_size.y *= s;
                    child.render(ui);
                });
            }
            Node::Empty => {}
            Node::Slider { value, min, max, on_change } => {
                let mut current = *value;
                if ui.add(egui::Slider::new(&mut current, *min..=*max)).changed() { (on_change)(current); }
            }
            Node::ScrollArea { children } => {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, child) in children.iter_mut().enumerate() {
                        ui.push_id(i, |ui| { child.render(ui); });
                    }
                });
            }
            Node::Separator => {
                ui.separator();
            }
            Node::ProgressBar { progress } => {
                ui.add(egui::ProgressBar::new(*progress));
            }
            Node::Toggle { checked, on_change } => {
                let mut current = *checked;
                // egui 0.36 has a toggle switch we can build using a selectable label or a custom widget, but let's try just a Checkbox for now or a custom toggle
                if ui.selectable_label(current, if current { "On" } else { "Off" }).clicked() {
                    (on_change)(!current);
                }
            }
            Node::Stepper { value, step, on_change } => {
                let mut current = *value as f64;
                if ui.add(egui::widgets::DragValue::new(&mut current).speed(*step as f64)).changed() {
                    (on_change)(current as f32);
                }
            }
            Node::ColorWell { color, on_change } => {
                let mut c = egui::Color32::from_rgba_premultiplied(color[0], color[1], color[2], color[3]);
                if ui.color_edit_button_srgba(&mut c).changed() {
                    (on_change)(c.to_array());
                }
            }
            Node::ComboBox { selected, options, on_change } => {
                let mut current = selected.clone();
                egui::ComboBox::from_id_salt(options.len()).selected_text(current.as_str()).show_ui(ui, |ui| {
                    for option in options {
                        if ui.selectable_value(&mut current, option.clone(), option.clone()).changed() {
                            (on_change)(current.clone());
                        }
                    }
                });
            }
            Node::SegmentedControl { selected, segments, on_change } => {
                ui.horizontal(|ui| {
                    let mut current = *selected;
                    for (i, segment) in segments.iter().enumerate() {
                        if ui.selectable_value(&mut current, i, segment.as_str()).changed() {
                            (on_change)(current);
                        }
                    }
                });
            }
            Node::Graph { points, min_x, max_x, min_y, max_y } => {
                let (response, painter) = ui.allocate_painter(egui::vec2(ui.available_width(), 300.0), egui::Sense::hover());
                let rect = response.rect;
                painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(30, 30, 30));
                
                let x_range = if *max_x > *min_x { *max_x - *min_x } else { 1.0 };
                let y_range = if *max_y > *min_y { *max_y - *min_y } else { 1.0 };
                let to_screen = |x: f32, y: f32| {
                    let rx = (x - *min_x) / x_range;
                    let ry = 1.0 - (y - *min_y) / y_range;
                    egui::pos2(rect.left() + rx * rect.width(), rect.top() + ry * rect.height())
                };
                
                if points.len() > 1 {
                    let screen_points: Vec<egui::Pos2> = points.iter().map(|(x, y)| to_screen(*x, *y)).collect();
                    let stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 200, 255));
                    painter.add(egui::Shape::line(screen_points, stroke));
                }
                
                let stroke_axis = egui::Stroke::new(1.0, egui::Color32::from_gray(100));
                if *min_x <= 0.0 && *max_x >= 0.0 {
                    painter.line_segment([to_screen(0.0, *min_y), to_screen(0.0, *max_y)], stroke_axis);
                }
                if *min_y <= 0.0 && *max_y >= 0.0 {
                    painter.line_segment([to_screen(*min_x, 0.0), to_screen(*max_x, 0.0)], stroke_axis);
                }
            }
        }
    }
}

impl Default for Node { fn default() -> Self { Node::Empty } }

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Text { text, .. } => f.debug_struct("Text").field("text", text).finish(),
            Node::TextInput { text, .. } => f.debug_struct("TextInput").field("text", text).finish(),
            Node::Checkbox { label, checked, .. } => f.debug_struct("Checkbox").field("label", label).field("checked", checked).finish(),
            Node::Button { label, .. } => f.debug_struct("Button").field("label", label).field("on_click", &"<closure>").finish(),
            Node::VStack { children, spacing } => f.debug_struct("VStack").field("children", children).field("spacing", spacing).finish(),
            Node::HStack { children, spacing } => f.debug_struct("HStack").field("children", children).field("spacing", spacing).finish(),
            Node::Empty => write!(f, "Empty"),
            Node::Center { child } => write!(f, "Center({:?})", child),
            Node::Scale { scale, child } => write!(f, "Scale({:?}, {:?})", scale, child),
            Node::Slider { value, min, max, .. } => f.debug_struct("Slider").field("value", value).field("min", min).field("max", max).finish(),
            Node::ScrollArea { children } => f.debug_struct("ScrollArea").field("children", children).finish(),
            Node::Separator => write!(f, "Separator"),
            Node::ProgressBar { progress } => f.debug_struct("ProgressBar").field("progress", progress).finish(),
            Node::Toggle { checked, .. } => f.debug_struct("Toggle").field("checked", checked).finish(),
            Node::Stepper { value, step, .. } => f.debug_struct("Stepper").field("value", value).field("step", step).finish(),
            Node::ColorWell { color, .. } => f.debug_struct("ColorWell").field("color", color).finish(),
            Node::ComboBox { selected, options, .. } => f.debug_struct("ComboBox").field("selected", selected).field("options", options).finish(),
            Node::SegmentedControl { selected, segments, .. } => f.debug_struct("SegmentedControl").field("selected", selected).field("segments", segments).finish(),
            Node::Graph { points, min_x, max_x, min_y, max_y } => f.debug_struct("Graph")
                .field("points_len", &points.len())
                .field("min_x", min_x).field("max_x", max_x)
                .field("min_y", min_y).field("max_y", max_y).finish(),
        }
    }
}
