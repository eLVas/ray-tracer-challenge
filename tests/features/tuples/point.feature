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

  Scenario: Multiplying a vector by a scalar
    Given v ← vector(1, -2, 3)
    Then v * 3.5 = vector(3.5, -7, 10.5)

  Scenario: Multiplying a vector by a fraction
    Given v ← vector(1, -2, 3)
    Then v * 0.5 = vector(0.5, -1, 1.5)

  Scenario: Dividing a vector by a scalar
    Given v ← vector(1, -2, 3)
    Then v / 2 = vector(0.5, -1, 1.5)

  Scenario: Computing the magnitude of vector(1, 0, 0)
    Given v ← vector(1, 0, 0)
    Then magnitude(v) = 1

  Scenario: Computing the magnitude of vector(0, 1, 0)
    Given v ← vector(0, 1, 0)
    Then magnitude(v) = 1

  Scenario: Computing the magnitude of vector(0, 0, 1)
    Given v ← vector(0, 0, 1)
    Then magnitude(v) = 1

  Scenario: Computing the magnitude of vector(1, 2, 3)
    Given v ← vector(1, 2, 3)
    Then magnitude(v) = √14

  Scenario: Computing the magnitude of vector(-1, -2, -3)
    Given v ← vector(-1, -2, -3)
    Then magnitude(v) = √14

  Scenario: Normalizing vector(4, 0, 0) gives (1, 0, 0)
    Given v ← vector(4, 0, 0)
    Then normalize(v) = vector(1, 0, 0)

  Scenario: Normalizing vector(1, 2, 3)
    Given v ← vector(1, 2, 3)
    Then normalize(v) = vector(0.26726, 0.53452, 0.80178)

  Scenario: The magnitude of a normalized vector
    Given v ← vector(1, 2, 3)
    When v ← normalize(v)
    Then magnitude(v) = 1

  Scenario: The dot product of two tuples
    Given v ← vector(1, 2, 3)
    And v2 ← vector(2, 3, 4)
    Then dot(v, v2) = 20

  Scenario: The cross product of two vectors
    Given v ← vector(1, 2, 3)
    And v2 ← vector(2, 3, 4)
    Then cross(v, v2) = vector(-1, 2, -1)
    And cross(v2, v) = vector(1, -2, 1)