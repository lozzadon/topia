use topia::Node;

#[test]
fn test_text_node_construction() {
    let node = Node::text("Hello, Topia!");
    assert_eq!(node.as_text(), Some("Hello, Topia!"));
    assert!(node.children().is_empty());
}

#[test]
fn test_button_node_construction() {
    let node = Node::button("Click Me", || {});
    assert_eq!(node.as_button_label(), Some("Click Me"));
    assert!(node.children().is_empty());
}

#[test]
fn test_empty_node_construction() {
    let node = Node::empty();
    assert!(node.is_empty());
    assert_eq!(node.as_text(), None);
    assert_eq!(node.as_button_label(), None);
    assert!(node.children().is_empty());
}

#[test]
fn test_vstack_node_construction() {
    let children = vec![
        Node::text("Line 1"),
        Node::text("Line 2"),
        Node::button("Action", || {}),
    ];
    let vstack = Node::vstack(children);

    assert_eq!(vstack.children().len(), 3);
    assert_eq!(vstack.children()[0].as_text(), Some("Line 1"));
    assert_eq!(vstack.children()[1].as_text(), Some("Line 2"));
    assert_eq!(vstack.children()[2].as_button_label(), Some("Action"));
}

#[test]
fn test_vstack_with_custom_spacing() {
    let node = Node::vstack_with_spacing(vec![Node::text("A"), Node::text("B")], 16.0);
    if let Node::VStack { spacing, children } = node {
        assert_eq!(spacing, Some(16.0));
        assert_eq!(children.len(), 2);
    } else {
        panic!("Expected VStack node");
    }
}

#[test]
fn test_hstack_node_construction() {
    let children = vec![
        Node::button("-", || {}),
        Node::text("0"),
        Node::button("+", || {}),
    ];
    let hstack = Node::hstack(children);

    assert_eq!(hstack.children().len(), 3);
    assert_eq!(hstack.children()[0].as_button_label(), Some("-"));
    assert_eq!(hstack.children()[1].as_text(), Some("0"));
    assert_eq!(hstack.children()[2].as_button_label(), Some("+"));
}

#[test]
fn test_hstack_with_custom_spacing() {
    let node = Node::hstack_with_spacing(vec![Node::text("X")], 8.5);
    if let Node::HStack { spacing, children } = node {
        assert_eq!(spacing, Some(8.5));
        assert_eq!(children.len(), 1);
    } else {
        panic!("Expected HStack node");
    }
}

#[test]
fn test_nested_counter_tree_construction() {
    let root = Node::vstack(vec![
        Node::text("Counter App"),
        Node::text("Value: 42"),
        Node::hstack(vec![
            Node::button("Decrement", || {}),
            Node::button("Increment", || {}),
            Node::button("Reset", || {}),
        ]),
        Node::empty(),
    ]);

    assert_eq!(root.children().len(), 4);
    assert_eq!(root.children()[0].as_text(), Some("Counter App"));
    assert_eq!(root.children()[1].as_text(), Some("Value: 42"));

    let inner_hstack = &root.children()[2];
    assert_eq!(inner_hstack.children().len(), 3);
    assert_eq!(inner_hstack.children()[0].as_button_label(), Some("Decrement"));
    assert_eq!(inner_hstack.children()[1].as_button_label(), Some("Increment"));
    assert_eq!(inner_hstack.children()[2].as_button_label(), Some("Reset"));

    assert!(root.children()[3].is_empty());
}

#[test]
fn test_node_debug_formatting() {
    let node = Node::vstack(vec![
        Node::text("Sample Text"),
        Node::button("Btn", || {}),
        Node::empty(),
    ]);
    let debug_str = format!("{:?}", node);
    assert!(debug_str.contains("VStack"));
    assert!(debug_str.contains("Sample Text"));
    assert!(debug_str.contains("Button"));
    assert!(debug_str.contains("Empty"));
}
