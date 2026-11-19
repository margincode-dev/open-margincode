Support log review (2026-11-06).

- A customer's email address showed up in the support tool, copied from a log line: the
  ValidationError raised by the region check had the rejected value in its message.
- Agreed with support: errors about customer data say what is wrong and set field=, and never
  include the value. This is for every check in validate.py (region, email, anything a customer
  typed), not only the one that leaked.
- The existing messages that still include the value are left as they are for now and fixed
  as each one is touched.
