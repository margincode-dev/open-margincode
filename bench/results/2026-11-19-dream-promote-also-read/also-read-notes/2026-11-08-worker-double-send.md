Notes from the session that looked at the mailer sending two emails per order (2026-11-08).

- Morning: the order-confirmation email went out twice for every order on the worker.
  I guessed the worker imports the mailer plugin twice, so its handler would be a second
  function object, and wrote the decision about deduping by __qualname__ on that guess.
  I did not confirm the double import.
- Afternoon: found the real cause. The plugin's setup() was called twice at worker start
  (once from the plugin loader, once from the old manual call in worker.py), and each call
  subscribed the same function object to "order.placed". Nothing was imported twice; it was
  the same object both times. Removing the manual call fixed the double send.
- Did not go back and change the decision record.
