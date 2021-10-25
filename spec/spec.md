```
use std::ui::{VStack, HStack, H1, Span};
use std::ui::theme;
use std::http::get;
use std::io::console;
use app::components::button::Button;

func logValue(value: i32) {
    console.log(value);
}

// Items can be made public by using the pub keyword.
pub comp MyApp(onClick: () -> ()) {
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

    onEffect {
        logValue(count);
        a += count;
    }

    // Whenever the component mounts, run this block
    onMount {
        console.log("component was mounted");
    }

    // Before the component updates, run this block
    beforeUpdate {
        console.log("component is about to update");
    }

    // After the component has update, run this block
    afterUpdated {
        console.log("component has updated");
    }

    // Before the component is about to be destroyed, run this block
    onDestroy {
        console.log("Component is about to be destroyed);
    }

    VStack(
        screen: {
            small: {spacing: theme.spacing.small}
            medium: {spacing: theme.spacing.medium},
            large: {spacing: theme.spacing.large}
        }
    ) {
        HStack(spacing: theme.spacing.xs) {
            H1("Count", color: theme.colors.gray500, fontWeight: theme.font.normal).

            Span(count, color: theme.colors.gray700, fontWeight: theme.font.bold)
        }

        Button(
            text: "Increment"
            onClick: increment,
        )
    }
}
```
