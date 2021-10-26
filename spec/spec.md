```
import std::ui::{VStack, HStack, H1, Span};
import std::ui::theme;
import std::http::get;
import std::io::console;
import app::components::button::Button;

func logValue(value: i32) {
    console.log(value);
}

// Items can be made public by using the pub keyword.
export comp MyApp(onClick: () -> ()) {
    // State variables are created using the state keyword.
    state count = 0;

    func increment() {
        // Assigning to a state variable will cause a rerender of a component.
        count += 1;
    }

    // Non-stateful variables are created with the let keyword
    let a = 0;

    // Non-stateful variables are immutable by default
    // Mutable variables can be created using the mut keyword.
    let mut b = 1;

    // An effectual block runs whenever any stateful variable inside it changes
    effect {
        logValue("The new count is \(count)");
        a += count;
    }

    // Whenever the component mounts, run this block
    onMount {
        console.log("component was mounted");
    }

    // After the component updates, run this block
    onUpdate {
        console.log("component is about to update");
    }

    // Before the component is about to be destroyed, run this block
    onDestroy {
        console.log("Component is about to be destroyed);
    }

    VStack {
        HStack {
            Profile()
        }
    }

    VStack(
        screen: {
            small: {spacing: theme.spacing.small}
            medium: {spacing: theme.spacing.medium},
            large: {spacing: theme.spacing.large}
        }
    ) {
        HStack(spacing: theme.spacing.xs) {
            H1("Count ${count}", color: theme.colors.gray500, fontWeight: theme.font.normal).
        }

        Button(
            text: "Increment"
            onClick: increment,
        )
    }
}
```
