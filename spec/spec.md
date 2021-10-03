```
use wlang::http;
use wlang::ui::{List, ForEach, Text};

struct User {
    name: String,
    email: String,
}

async fn fetchUsers() -> Result<Vec<User>, http::Error> {
    return http::get("/users").decode(User);
}

pub async component UserList() {
    let users = fetchUsers().await;

    if let Error(err) = users {
        return Text("Oh no, something went wrong");
    }

    return List(|| {
        ForEach(users, |user| {
            Text(user.name).color("gray-700);
            Text(user.email).color("gray-300);
        })
    })
}
```
