// Day 1 2019

#[cfg(test)]
mod tests {
    use crate::day1::*;
    #[test]
    fn test_fuel_from_mass() {
        assert_eq!(fuel_from_mass(12), 2);
        assert_eq!(fuel_from_mass(14), 2);
        assert_eq!(fuel_from_mass(1969), 654);
        assert_eq!(fuel_from_mass(100756), 33583);
    }
    #[test]
    fn test_fuel_for_module() {
        assert_eq!(fuel_for_module(14), 2);
        assert_eq!(fuel_for_module(1969), 966);
        assert_eq!(fuel_for_module(100756), 50346);
    }
}

/// Gets the amount of fuel required for a module based on it's mass.
pub fn fuel_from_mass(mass: usize) -> usize {
    let divided = mass.div_euclid(3);
    if divided > 2 {
        divided - 2
    } else {
        0
    }
}

/// Gets the amount of fuel required for a module *accounting for fuel*
pub fn fuel_for_module(mass: usize) -> usize {
    // The fuel from the mass of the module alone
    let mut fuel = fuel_from_mass(mass);
    // The fuel cost for the fuel
    let mut fuel_cost = fuel_from_mass(fuel);
    // Add the fuel cost to the fuel total, than set the fuel cost to the cost of the fuel cost
    while fuel_cost > 0 {
        fuel += fuel_cost;
        fuel_cost = fuel_from_mass(fuel_cost);
    }

    fuel
}
