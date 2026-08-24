use topia::{App, Node, TopiaApp, TopiaEframeApp};

#[test]
fn test_app_struct_initialization_defaults() {
    let app = App::new("Counter Window", 400.0, 300.0);
    assert_eq!(app.title, "Counter Window");
    assert_eq!(app.width, 400.0);
    assert_eq!(app.height, 300.0);
    assert!(app.resizable, "Default resizable should be true");
}

#[test]
fn test_app_fluent_builder_configuration() {
    let app = App::new("Initial", 100.0, 100.0)
        .with_title("Configured Window")
        .with_size(800.0, 600.0)
        .with_resizable(false);

    assert_eq!(app.title, "Configured Window");
    assert_eq!(app.width, 800.0);
    assert_eq!(app.height, 600.0);
    assert!(!app.resizable);
}

#[test]
fn test_app_resizable_alias() {
    let app = App::new("Test", 200.0, 200.0).resizable(false);
    assert!(!app.resizable);

    let app2 = app.resizable(true);
    assert!(app2.resizable);
}

#[test]
fn test_topia_app_instantiation_and_eval() {
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

#[test]
fn test_topia_eframe_app_alias() {
    let mut app = TopiaEframeApp::new(|| Node::text("Alias Test"));
    let node = (app.view_builder)();
    assert_eq!(node.as_text(), Some("Alias Test"));
}
