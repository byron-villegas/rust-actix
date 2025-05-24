Feature: Products
    Scenario: Gets Products
        Given An endpoint /products
        When I send a GET request to the endpoint
        Then I should receive a 200 status code
    Scenario Outline: Get Product By SKU
        Given An endpoint /products/<SKU>
        When I send a GET request to the endpoint
        Then I should receive a <STATUSCODE> status code
    Examples:
    | SKU      | STATUSCODE |
    | 15207410 | 200        |
    | 15207411 | 404        |