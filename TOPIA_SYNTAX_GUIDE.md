# Topia Syntax Guide

Welcome to the official syntax guide for **Topia**, the declarative desktop UI framework for `f(x)`.

Topia follows a reactive, immediate-mode GUI paradigm (powered internally by `egui`). This means your UI is a direct reflection of your state. Whenever an interaction occurs, Topia re-evaluates your UI builder closure to render the latest state.

---

## 1. Importing Topia

To use Topia in your `f(x)` scripts, import it from the standard library:

```fx
let ui = import("std:topia")
```

---

## 2. The Application Lifecycle

Every Topia application requires two things: an `App` configuration, and a `run` loop that takes a view builder closure.

```fx
let ui = import("std:topia")

let app_builder = func() {
    // Your UI components go here
    ui.Text("Hello, Topia!")
}

// Create an app window titled "My App" with dimensions 800x600
let app = ui.App("My App", 800, 600)

// Launch the native window and start the render loop
ui.run(app, app_builder)
```

---

## 3. Core Widgets

Topia provides several built-in widgets for displaying information and capturing user input.

### `ui.Text`
Displays static or dynamic text. You can optionally pass a styling hash to configure its appearance.

```fx
// Standard text
ui.Text("This is normal text.")

// Styled text
ui.Text("This is a heading!", {"size": 24, "bold": true})
```

### `ui.Button`
A clickable button that executes a closure.

```fx
var count = 0

ui.Button("Increment Count", func() {
    count = count + 1
})
```

### `ui.TextInput`
A single-line text input field. It requires the current string value and a callback closure that receives the new string whenever the user types.

```fx
var username = ""

ui.TextInput(username, func(new_val) {
    username = new_val
})
```

### `ui.Checkbox`
A toggleable checkbox. It requires the current boolean state, a label, and a callback closure that receives the toggled state.

```fx
var is_active = false

ui.Checkbox(is_active, "Enable Feature", func(new_val) {
    is_active = new_val
})
```

---

## 4. Layouts

You can compose complex interfaces by nesting layouts. Layouts accept an array of child widgets.

### `ui.VStack` (Vertical Stack)
Arranges its children from top to bottom.

```fx
ui.VStack([
    ui.Text("Row 1"),
    ui.Text("Row 2"),
    ui.Text("Row 3")
])
```

### `ui.HStack` (Horizontal Stack)
Arranges its children from left to right.

```fx
ui.HStack([
    ui.Button("Cancel", func() {}),
    ui.Button("Submit", func() {})
])
```

---

## 5. Full Example: Interactive Todo List

Here is how all the pieces come together to create a fully reactive desktop application:

```fx
let ui = import("std:topia")

var tasks = []
var new_task_name = ""

let add_task = func() {
    if len(new_task_name) > 0 {
        push(tasks, {"name": new_task_name, "completed": false})
        new_task_name = ""
    }
}

let render = func() {
    var task_nodes = []
    
    // Build the list of task checkboxes
    var i = 0
    while i < len(tasks) {
        let idx = i
        let task = tasks[idx]
        
        let on_toggle = func(checked) {
            tasks[idx].completed = checked
        }
        
        push(task_nodes, ui.Checkbox(task.completed, task.name, on_toggle))
        i = i + 1
    }
    
    // Combine everything into a vertical layout
    ui.VStack([
        ui.Text("My Tasks", {"size": 32, "bold": true}),
        
        // Input Area
        ui.HStack([
            ui.TextInput(new_task_name, func(val) { new_task_name = val }),
            ui.Button("Add Task", add_task)
        ]),
        
        // Task List
        ui.VStack(task_nodes)
    ])
}

let app = ui.App("Task Manager", 400, 500)
ui.run(app, render)
```
