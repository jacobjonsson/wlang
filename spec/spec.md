```
// The props block defines what arguments can be provided to the component
props {
    name: string
    children:
}

// The script block defines runtime behavior
script {
    // A stateful variable is created with the state keyword.
    state a: u8 = 1;

    // A non-stateful variable is created with the let keyword.
    let b: u8 = 2;

    // Non-stateful variables are immutable by default, to make it mutable, use the mut keyword.
    let mut c: u8 = 3;

    // Functions are created with the fn keyword.
    fn increment() {
        // Assigning to a stateful keyword will
        a += 1;
    }

    // Effect blocks run whenever a stateful variable referenced inside the block changes
    effect {
        // Will be executed whenever the stateful variable a changes
        console.log(a);
    }

    // The mount blocks run when the component first mounts
    mount {
        console.log("Hello world");
    }

    // The update blocks run when the component updates
    update {
        console.log("Hello world, again");
    }

    // The destroy blocks run before the component will be destroyed
    destroy {
        console.log("Bye bye world");
    }
}

// The view block defines the html
view {
    <h1 on:>Hello {props.name}</h1>

    <button>Click to increment</button>

    <p>Count</p>

    {#if a > 10}
        <p>You hit the jackpot!!</p>
    {/if}
}

// The style block defined the css
// Styles defined inside of this block is scoped to this component
style {
    .hello {
        background-color: "red";
    }
}
```
