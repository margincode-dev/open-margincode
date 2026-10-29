import pytest

from tallyho import Customer
from tallyho.errors import ValidationError


@pytest.mark.parametrize("region", ["us-ca", "USA-CA", "U", "US_CA"])
def test_invalid_regions_are_rejected_without_echoing_them(region):
    with pytest.raises(ValidationError) as e:
        Customer("c1", "Alice", "alice@example.com", region)
    assert e.value.field == "region"
    assert region not in str(e.value)
    assert region.lower() not in str(e.value).lower()


@pytest.mark.parametrize("region", ["US-CA", "GB", "DE", "US-NY"])
def test_valid_regions_are_accepted(region):
    assert Customer("c1", "Alice", "alice@example.com", region).region == region
