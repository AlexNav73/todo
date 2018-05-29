const rust = import('./todo_client');

rust.then(m => m.greet("world"));
