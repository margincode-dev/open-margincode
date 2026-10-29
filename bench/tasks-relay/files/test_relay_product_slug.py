from tallyho import Money, Product


def test_slug_leads_with_the_sku():
    assert Product("MUG-1", "Blue mug", Money.of("1")).slug == "mug-1-blue-mug"
    assert Product("MUG-2", "Blue Mug (Large)", Money.of("1")).slug == "mug-2-blue-mug-large"


def test_slugs_of_products_with_the_same_name_differ():
    a = Product("MUG-1", "Blue mug", Money.of("1"))
    b = Product("MUG-9", "Blue mug", Money.of("1"))
    assert a.slug != b.slug
