fn main() {
    // TODO
    // get all by user
    // get todo list by user
    // create a todo
    // edit a todo
    // make as completed
    // delete a todo

    // register a user
    // login
    println!("Hello, world!");

    /*
    request
        -> router
            -> handler (aka controller)
                extractor (Path, Query)
                -> usecases (business logic)
                   models (aka transformer)
                    -> repositories
                        entities (aka table schema)
                        -> database
                response (IntoResponse)

    error handlering
    middleware (ServiceBuilder is recommended)
    
    Sharing state with handlers (share database connection)
    - State extractor >> .with_state(shared_state)
    - Request extensions >> .layer(Extension(shared_state))
    - Closure captures (ข้อเสีย เขียนเยอะ)
    ```rust
    let shared_state = Arc::new(AppState { /* ... */ });

    post({
            let shared_state = Arc::clone(&shared_state);
            move |body| create_user(body, shared_state)
        }),
    ```
    - task-local variables (smol does not yet support)

     */
}
