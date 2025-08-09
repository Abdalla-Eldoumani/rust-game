Goal: Practice async fan-in with Tokio mpsc.

Key ideas:
- Create a channel, spawn tasks that send values, drop the sender, and collect.
- Use #[tokio::test] in tests for async context.

Why this matters: Teaches coordination of async tasks with message passing.


