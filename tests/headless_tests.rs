use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use topia::{App, Node};

#[test]
fn test_node_constructors_and_tree_nesting() {
    let clicked = Arc::new(AtomicUsize::new(0));
    let clicked_clone = Arc::clone(&clicked);

    let mut tree = Node::vstack_with_spacing(
        vec![
            Node::text("Counter App"),
            Node::hstack_with_spacing(
                vec![
                    Node::button("Increment", move || {
                        clicked_clone.fetch_add(1, Ordering::SeqCst);
                    }),
                    Node::button("Decrement", || {}),
                ],
                8.0,
            ),
            Node::empty(),
        ],
        12.0,
    );

    if let Node::VStack { children, spacing } = &mut tree {
        assert_eq!(children.len(), 3);
        assert_eq!(*spacing, Some(12.0));

        if let Node::Text { text, .. } = &children[0] {
            assert_eq!(text, "Counter App");
        } else {
            panic!("Expected Text node");
        }

        if let Node::HStack {
            children: h_children,
            spacing: h_spacing,
        } = &mut children[1]
        {
            assert_eq!(h_children.len(), 2);
            assert_eq!(*h_spacing, Some(8.0));

            if let Node::Button { label, on_click } = &mut h_children[0] {
                assert_eq!(label, "Increment");
                on_click();
            } else {
                panic!("Expected Button node");
            }
        } else {
            panic!("Expected HStack node");
        }

        if let Node::Empty = &children[2] {
            // verified
        } else {
            panic!("Expected Empty node");
        }
    } else {
        panic!("Expected VStack node");
    }

    assert_eq!(clicked.load(Ordering::SeqCst), 1);
}

#[test]
fn test_app_new() {
    let app = App::new("Counter Window", 400.0, 300.0);
    assert_eq!(app.title, "Counter Window");
    assert_eq!(app.width, 400.0);
    assert_eq!(app.height, 300.0);
    assert!(app.resizable);
}

#[test]
fn test_debug_impl() {
    let node = Node::button("Click me", || {});
    let formatted = format!("{:?}", node);
    assert!(formatted.contains("Button"));
    assert!(formatted.contains("Click me"));
}
