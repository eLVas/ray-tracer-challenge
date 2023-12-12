Feature: Point representation

  Scenario: A tuple with w=1.0 is a point
    Given a ← tuple(4.3, -4.2, 3.1, 1.0)
    Then a.x = 4.3
    And a.y = -4.2
    And a.z = 3.1
    And a.w = 1.0
    And a is a point
    And a is not a vector

  Scenario: A tuple with w=0.0 is a vector
    Given a ← tuple(4.3, -4.2, 3.1, 0.0)
    Then a.x = 4.3
    And a.y = -4.2
    And a.z = 3.1
    And a.w = 0.0
    And a is not a point
    And a is a vector

  Scenario: Point::new() creates tuples with w=1
    Given p ← point(4, -4, 3)
    And a ← point.tuple
    Then p = tuple(4, -4, 3, 1)
    And a is a point
    And a is not a vector

  Scenario: Vector::new() creates tuples with w=0
    Given v ← vector(4, -4, 3)
    And a ← vector.tuple
    Then v = tuple(4, -4, 3, 0)
    And a is not a point
    And a is a vector

  Scenario: Adding vector to point
    Given p ← point(3, -2, 5)
    And v ← vector(-2, 3, 1)
    Then p + v = point(1, 1, 6)

  Scenario: Adding vector to vector
    Given v ← vector(3, -2, 5)
    And v2 ← vector(-2, 3, 1)
    Then v + v2 = vector(1, 1, 6)

  Scenario: Subtracting point from other point
    Given p ← point(3, 2, 1)
    And p2 ← point(5, 6, 7)
    Then p - p2 = vector(-2, -4, -6)

  Scenario: Subtracting a vector from a point
    Given p ← point(3, 2, 1)
    And v ← vector(5, 6, 7)
    Then p - v = point(-2, -4, -6)

  Scenario: Subtracting two vectors
    Given v ← vector(3, 2, 1)
    And v2 ← vector(5, 6, 7)
    Then v - v2 = vector(-2, -4, -6)