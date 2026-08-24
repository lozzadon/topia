use eframe::egui;
use topia::{Node, TopiaApp};

#[test]
fn test_headless_egui_rendering_empty_node() {
    let ctx = egui::Context::default();
    let mut empty_node = Node::empty();

    let mut output = ctx.run_ui(Default::default(), |ui| {
        empty_node.render(ui);
    });
    output.textures_delta.clear();
}

#[test]
fn test_headless_egui_rendering_empty_stacks() {
    let ctx = egui::Context::default();
    let mut empty_vstack = Node::vstack(vec![]);
    let mut empty_hstack = Node::hstack(vec![]);
    let mut vstack_with_spacing = Node::vstack_with_spacing(vec![], 20.0);
    let mut hstack_with_spacing = Node::hstack_with_spacing(vec![], 15.0);

    let mut output = ctx.run_ui(Default::default(), |ui| {
        empty_vstack.render(ui);
        empty_hstack.render(ui);
        vstack_with_spacing.render(ui);
        hstack_with_spacing.render(ui);
    });
    output.textures_delta.clear();
}

#[test]
fn test_headless_egui_rendering_deeply_nested_tree() {
    let ctx = egui::Context::default();
    let mut tree = Node::text("Leaf content");
    for i in 0..60 {
        if i % 2 == 0 {
            tree = Node::vstack_with_spacing(vec![tree], 2.0);
        } else {
            tree = Node::hstack_with_spacing(vec![tree], 3.0);
        }
    }

    let mut output = ctx.run_ui(Default::default(), |ui| {
        tree.render(ui);
    });
    output.textures_delta.clear();
}

#[test]
fn test_headless_egui_rendering_wide_layout() {
    let ctx = egui::Context::default();
    let children: Vec<Node> = (0..200)
        .map(|i| {
            if i % 2 == 0 {
                Node::text(format!("Text Item {}", i))
            } else {
                Node::button(format!("Button {}", i), || {})
            }
        })
        .collect();

    let mut vstack = Node::vstack(children);

    let mut output = ctx.run_ui(Default::default(), |ui| {
        vstack.render(ui);
    });
    output.textures_delta.clear();
}

#[test]
fn test_headless_egui_rendering_unicode_and_multiline() {
    let ctx = egui::Context::default();
    let mut complex_node = Node::vstack(vec![
        Node::text("Line 1\nLine 2\nLine 3 with unicode: 🚀 🦀 🌈 🎯"),
        Node::button("Unicode Button 🔥 \t\n with tabs", || {}),
        Node::empty(),
    ]);

    let mut output = ctx.run_ui(Default::default(), |ui| {
        complex_node.render(ui);
    });
    output.textures_delta.clear();
}

#[test]
fn test_headless_topia_app_render_frame() {
    let ctx = egui::Context::default();
    let mut count = 0;
    let mut app = TopiaApp::new(move || {
        count += 1;
        Node::vstack(vec![
            Node::text(format!("Render pass {}", count)),
            Node::button("Click", || {}),
        ])
    });

    for _ in 0..10 {
        let mut output = ctx.run_ui(Default::default(), |ui| {
            app.render_frame(ui);
        });
        output.textures_delta.clear();
    }
}
