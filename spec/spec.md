```
<script>
    // A prop is created with the prop keyword.
    prop value: string;

    // A stateful variable is created with the state keyword.
    state counter: i32 = 0;

    // A non-stateful variable is created with the let keyword.
    let b: u8 = 2;

    // Non-stateful variables are immutable by default, to make it mutable, use the mut keyword.
    let mut c: u8 = 3;

    // Functions are created with the fn keyword.
    fn increment() {
        // Assigning to a stateful variable will trigger the component to rerender
        counter += 1;
    }

    // Effect blocks run whenever a stateful variable referenced inside the block updates
    effect {
        // Will be executed whenever the stateful variable a changes
        console.log(counter);
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
</script>

// The style tag defines component scoped css
<style>
    button {
        padding: 16px;
        background-color: blue;
        font-color: white;
    }
</style>

<button className="button" on:click={increment}>
    // You can reference any declaration from the script block by wrapping it in curly braces
    Count: {count}
</button>

// The children tag can be used to render whatever children gets passed to the component
<children />
```
