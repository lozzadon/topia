use topia::{App, Node};

#[test]
fn test_empty_node_invariants() {
    let mut empty = Node::empty();
    assert!(empty.is_empty());
    assert_eq!(empty.as_text(), None);
    assert_eq!(empty.text_content(), None);
    assert_eq!(empty.as_button_label(), None);
    assert_eq!(empty.button_label(), None);
    assert_eq!(empty.child_count(), 0);
    assert!(empty.children().is_empty());
    assert!(!empty.fire_click());
    assert!(!empty.trigger_click());

    // Default trait invariant
    let default_node: Node = Default::default();
    assert!(default_node.is_empty());
    assert_eq!(format!("{:?}", default_node), "Empty");
}

#[test]
fn test_empty_stacks_invariants() {
    // Empty VStack with default spacing
    let mut empty_vstack = Node::vstack(vec![]);
    assert!(!empty_vstack.is_empty());
    assert_eq!(empty_vstack.child_count(), 0);
    assert!(empty_vstack.children().is_empty());
    assert_eq!(empty_vstack.as_text(), None);
    assert_eq!(empty_vstack.as_button_label(), None);
    assert!(!empty_vstack.fire_click());
    if let Node::VStack { children, spacing } = &empty_vstack {
        assert!(children.is_empty());
        assert_eq!(*spacing, None);
    } else {
        panic!("Expected VStack");
    }

    // Empty VStack with custom spacing (including zero and negative)
    let vstack_zero = Node::vstack_with_spacing(vec![], 0.0);
    assert_eq!(vstack_zero.child_count(), 0);
    if let Node::VStack { spacing, .. } = vstack_zero {
        assert_eq!(spacing, Some(0.0));
    }

    let vstack_neg = Node::vstack_with_spacing(vec![], -10.5);
    if let Node::VStack { spacing, .. } = vstack_neg {
        assert_eq!(spacing, Some(-10.5));
    }

    // Empty HStack with default spacing
    let mut empty_hstack = Node::hstack(vec![]);
    assert!(!empty_hstack.is_empty());
    assert_eq!(empty_hstack.child_count(), 0);
    assert!(empty_hstack.children().is_empty());
    assert_eq!(empty_hstack.as_text(), None);
    assert_eq!(empty_hstack.as_button_label(), None);
    assert!(!empty_hstack.fire_click());

    // Empty HStack with custom spacing
    let hstack_custom = Node::hstack_with_spacing(vec![], 99.9);
    if let Node::HStack { spacing, children } = hstack_custom {
        assert!(children.is_empty());
        assert_eq!(spacing, Some(99.9));
    }
}

#[test]
fn test_stacks_containing_only_empty_nodes() {
    let empty_children = vec![Node::empty(), Node::empty(), Node::empty(), Node::empty()];
    let vstack = Node::vstack(empty_children);
    assert_eq!(vstack.child_count(), 4);
    for child in vstack.children() {
        assert!(child.is_empty());
        assert_eq!(child.child_count(), 0);
    }

    let hstack = Node::hstack(vec![Node::empty()]);
    assert_eq!(hstack.child_count(), 1);
    assert!(hstack.children()[0].is_empty());
}

#[test]
fn test_deeply_nested_empty_containers() {
    let mut root = Node::empty();
    for i in 0..50 {
        if i % 2 == 0 {
            root = Node::vstack(vec![root]);
        } else {
            root = Node::hstack(vec![root]);
        }
    }

    // Traverse down to verify 50 levels of single child ending in Empty
    let mut curr = &root;
    let mut depth = 0;
    while curr.child_count() > 0 {
        depth += 1;
        curr = &curr.children()[0];
    }
    assert_eq!(depth, 50);
    assert!(curr.is_empty());
}

#[test]
fn test_empty_and_special_strings() {
    // Empty text
    let empty_text = Node::text("");
    assert_eq!(empty_text.as_text(), Some(""));
    assert_eq!(empty_text.text_content(), Some(""));
    assert_eq!(empty_text.child_count(), 0);
    assert!(!empty_text.is_empty()); // It's Node::Text, not Node::Empty

    // Empty button label
    let mut empty_btn = Node::button("", || {});
    assert_eq!(empty_btn.as_button_label(), Some(""));
    assert_eq!(empty_btn.button_label(), Some(""));
    assert!(empty_btn.fire_click());

    // Unicode, emoji, newlines, tabs, zero bytes
    let special_str = "🔥 UTF-8 Test: \n\t\r \"quotes\" \\ backslash \u{1F980} \u{0000} end";
    let special_text = Node::text(special_str);
    assert_eq!(special_text.as_text(), Some(special_str));

    let special_btn = Node::button(special_str, || {});
    assert_eq!(special_btn.as_button_label(), Some(special_str));

    // Large string (100,000 characters)
    let large_str = "A".repeat(100_000);
    let large_text = Node::text(large_str.clone());
    assert_eq!(large_text.as_text().map(|s| s.len()), Some(100_000));
}

#[test]
fn test_debug_formatting_edge_cases() {
    // Empty
    assert_eq!(format!("{:?}", Node::empty()), "Empty");

    // Empty Text
    let debug_empty_text = format!("{:?}", Node::text(""));
    assert_eq!(debug_empty_text, "Text { text: \"\" }");

    // Empty Button
    let debug_empty_btn = format!("{:?}", Node::button("", || {}));
    assert_eq!(
        debug_empty_btn,
        "Button { label: \"\", on_click: \"<closure>\" }"
    );

    // Empty VStack and HStack
    let debug_empty_vstack = format!("{:?}", Node::vstack(vec![]));
    assert_eq!(
        debug_empty_vstack,
        "VStack { children: [], spacing: None }"
    );

    let debug_empty_hstack = format!("{:?}", Node::hstack_with_spacing(vec![], 4.5));
    assert_eq!(
        debug_empty_hstack,
        "HStack { children: [], spacing: Some(4.5) }"
    );

    // Multiline & special characters in text and button debug output
    let text_with_escapes = Node::text("line1\nline2\t\"quoted\"");
    let debug_escapes = format!("{:?}", text_with_escapes);
    assert!(debug_escapes.contains(r#"text: "line1\nline2\t\"quoted\""#));

    // App debug formatting
    let app = App::new("My Title", 640.0, 480.0).with_resizable(false);
    let debug_app = format!("{:?}", app);
    assert!(debug_app.contains("App"));
    assert!(debug_app.contains("title: \"My Title\""));
    assert!(debug_app.contains("width: 640.0"));
    assert!(debug_app.contains("height: 480.0"));
    assert!(debug_app.contains("resizable: false"));
}

#[test]
fn test_app_configuration_corner_cases() {
    // Zero dimensions
    let app_zero = App::new("", 0.0, 0.0);
    assert_eq!(app_zero.title, "");
    assert_eq!(app_zero.width, 0.0);
    assert_eq!(app_zero.height, 0.0);
    assert!(app_zero.resizable);

    // Negative dimensions
    let app_neg = App::new("Negative", -500.0, -300.0);
    assert_eq!(app_neg.width, -500.0);
    assert_eq!(app_neg.height, -300.0);

    // Extreme dimensions
    let app_huge = App::new("Huge", f32::MAX, f32::MAX);
    assert_eq!(app_huge.width, f32::MAX);
    assert_eq!(app_huge.height, f32::MAX);

    // Fluent method chaining override order
    let app_chained = App::new("Original", 100.0, 100.0)
        .with_title("First Title")
        .with_title("Second Title")
        .with_size(200.0, 300.0)
        .with_size(400.0, 500.0)
        .with_resizable(false)
        .resizable(true);

    assert_eq!(app_chained.title, "Second Title");
    assert_eq!(app_chained.width, 400.0);
    assert_eq!(app_chained.height, 500.0);
    assert!(app_chained.resizable);

    // Clone independence
    let app_clone = app_chained.clone();
    assert_eq!(app_clone.title, app_chained.title);
    assert_eq!(app_clone.width, app_chained.width);
    assert_eq!(app_clone.height, app_chained.height);
    assert_eq!(app_clone.resizable, app_chained.resizable);
}
