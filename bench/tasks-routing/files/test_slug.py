"""Hidden check: slugify collapses runs and strips hyphens."""

from slugify import slugify

assert slugify("Hello  World") == "hello-world"
assert slugify(" a ") == "a"
assert slugify("A--B") == "a-b"
assert slugify("") == ""
assert slugify("...") == ""
assert slugify("x---y  z") == "x-y-z"
print("ok")
