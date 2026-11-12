from tallyho.events import EventBus


def test_a_handler_subscribed_twice_runs_once():
    bus, seen = EventBus(), []

    def on_paid(event, payload):
        seen.append((event, payload["id"]))

    bus.subscribe("order.paid", on_paid)
    bus.subscribe("order.paid", on_paid)
    bus.publish("order.paid", {"id": "o1"})
    assert seen == [("order.paid", "o1")]


def test_one_handler_on_two_events_gets_both():
    bus, seen = EventBus(), []

    def on_any(event, payload):
        seen.append(event)

    bus.subscribe("order.paid", on_any)
    bus.subscribe("order.shipped", on_any)
    bus.subscribe("order.shipped", on_any)
    bus.publish("order.paid", {"id": "o1"})
    bus.publish("order.shipped", {"id": "o1"})
    assert seen == ["order.paid", "order.shipped"]


def test_different_handlers_on_one_event_all_run():
    bus, seen = EventBus(), []

    def notifier(channel):
        def handle(event, payload):
            seen.append((channel, payload["id"]))

        return handle

    bus.subscribe("order.paid", notifier("email"))
    bus.subscribe("order.paid", notifier("sms"))
    bus.subscribe("order.paid", lambda event, payload: seen.append(("audit", payload["id"])))
    bus.subscribe("order.paid", lambda event, payload: seen.append(("stock", payload["id"])))
    bus.publish("order.paid", {"id": "o1"})
    assert sorted(seen) == [("audit", "o1"), ("email", "o1"), ("sms", "o1"), ("stock", "o1")]


def test_the_same_method_of_two_objects_is_two_handlers():
    class Tally:
        def __init__(self):
            self.count = 0

        def on_paid(self, event, payload):
            self.count += 1

    bus, shop, warehouse = EventBus(), Tally(), Tally()
    bus.subscribe("order.paid", shop.on_paid)
    bus.subscribe("order.paid", warehouse.on_paid)
    bus.publish("order.paid", {"id": "o1"})
    assert (shop.count, warehouse.count) == (1, 1)
