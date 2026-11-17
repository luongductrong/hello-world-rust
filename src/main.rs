// mod lesson_1;
// use lesson_1::run;
// mod guessing_game;

// mod lesson_2;

// mod lesson_3;
// mod lesson_3_practice;
// mod vending_machine;

// mod lesson_4_collections;
// mod lesson_4_iterators;
// mod lesson_4_errors;

// mod lesson_5_traits;
// mod lesson_5_generics;
// mod lesson_5_dispatch;
// mod todo_list_cli;

// mod lesson_6_lifetimes;
// mod lesson_6_associated_types;

// mod lesson_7_threads;
// mod lesson_7_interior_mutability;
// mod lesson_7_shared_state;

// mod lesson_8_thiserror;
// mod lesson_8_anyhow;

// mod lesson_9_async_basics;
mod lesson_9_tasks;

#[tokio::main]
async fn main() {
    lesson_9_tasks::run().await;
}
