```
use std::ui::{VStack, HStack, Text, Button as BButton};
use http::get;

view Button(onClick: () -> ()) {
    state count = 0;

    fn increment() {
        count += 1;
    }

    VStack(spacing = ) {
        Text("Count" + count, element = "h1").

        Button(onClick = increment) {

        }
    }
}
```
