# Nearby Coordinate
This project was inspired by inspired by [faker js nearby coordinate function](https://github.com/faker-js/faker/blob/b8abfc6415fe5be3a207b1b3dd4266905b924f84/src/modules/location/index.ts#L131).

# Usage

```rust
let nearby_coordinate =
    NearbyCoordinate::new(41.896738135197026, -87.62393942418863, 10.0, false); // latitude, longitude, radius, miles?
let coordinate = nearby_coordinate.get_random_coordinate();
```
