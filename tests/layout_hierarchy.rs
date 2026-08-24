use topia::Node;

/// Helper to count total nodes in a tree recursively.
fn count_total_nodes(node: &Node) -> usize {
    1 + node.children().iter().map(count_total_nodes).sum::<usize>()
}

/// Helper to compute maximum depth of a tree.
fn compute_max_depth(node: &Node) -> usize {
    let child_depth = node.children().iter().map(compute_max_depth).max().unwrap_or(0);
    1 + child_depth
}

/// Helper to collect all text strings in tree in pre-order traversal.
fn collect_all_texts(node: &Node) -> Vec<String> {
    let mut texts = Vec::new();
    if let Some(t) = node.as_text() {
        texts.push(t.to_string());
    }
    for child in node.children() {
        texts.extend(collect_all_texts(child));
    }
    texts
}

/// Helper to collect all button labels in tree.
fn collect_all_button_labels(node: &Node) -> Vec<String> {
    let mut labels = Vec::new();
    if let Some(l) = node.as_button_label() {
        labels.push(l.to_string());
    }
    for child in node.children() {
        labels.extend(collect_all_button_labels(child));
    }
    labels
}

#[test]
fn test_single_leaf_depth_and_count() {
    let text_leaf = Node::text("Leaf");
    assert_eq!(count_total_nodes(&text_leaf), 1);
    assert_eq!(compute_max_depth(&text_leaf), 1);

    let btn_leaf = Node::button("Leaf Btn", || {});
    assert_eq!(count_total_nodes(&btn_leaf), 1);
    assert_eq!(compute_max_depth(&btn_leaf), 1);

    let empty_leaf = Node::empty();
    assert_eq!(count_total_nodes(&empty_leaf), 1);
    assert_eq!(compute_max_depth(&empty_leaf), 1);
}

#[test]
fn test_flat_container_hierarchy() {
    let vstack = Node::vstack(vec![
        Node::text("Item 1"),
        Node::text("Item 2"),
        Node::text("Item 3"),
    ]);

    assert_eq!(count_total_nodes(&vstack), 4); // 1 parent + 3 children
    assert_eq!(compute_max_depth(&vstack), 2);
    assert_eq!(
        collect_all_texts(&vstack),
        vec!["Item 1", "Item 2", "Item 3"]
    );
}

#[test]
fn test_multi_level_nesting_hierarchy() {
    let root = Node::vstack(vec![
        Node::text("Header"),
        Node::hstack(vec![
            Node::vstack(vec![
                Node::button("Nested Button", || {}),
            ]),
            Node::text("Sidebar Info"),
        ]),
    ]);

    assert_eq!(count_total_nodes(&root), 6);
    assert_eq!(compute_max_depth(&root), 4);
    assert_eq!(collect_all_texts(&root), vec!["Header", "Sidebar Info"]);
    assert_eq!(collect_all_button_labels(&root), vec!["Nested Button"]);
}

#[test]
fn test_deeply_nested_stacks() {
    let mut current = Node::text("Deepest Element");
    for _ in 0..24 {
        current = Node::vstack(vec![current]);
    }

    assert_eq!(compute_max_depth(&current), 25);
    assert_eq!(count_total_nodes(&current), 25);
    assert_eq!(collect_all_texts(&current), vec!["Deepest Element"]);
}

#[test]
fn test_wide_container_layout() {
    let children: Vec<Node> = (0..50)
        .map(|i| Node::text(format!("Row {}", i)))
        .collect();
    let container = Node::vstack(children);

    assert_eq!(count_total_nodes(&container), 51);
    assert_eq!(compute_max_depth(&container), 2);
    let texts = collect_all_texts(&container);
    assert_eq!(texts.len(), 50);
    assert_eq!(texts[0], "Row 0");
    assert_eq!(texts[49], "Row 49");
}
