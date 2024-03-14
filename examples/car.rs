#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Car
{
    /// `A` <br>
    /// name: &str = `Jonda` <br>
    /// is_expensive: bool = `false` <br>
    A,
    
    /// `B` <br>
    /// name: &str = `Gord` <br>
    /// is_expensive: bool = `false` <br>
    B,
    
    /// `C` <br>
    /// name: &str = `Hamborghini` <br>
    /// is_expensive: bool = `true` <br>
    C,
    
    /// `D` <br>
    /// name: &str = `Yotota` <br>
    /// is_expensive: bool = `true` <br>
    D,
    
}
impl Car
{
    pub fn get_all_variants() -> [Self; 4]
    {
        car_get_all_variants()
    }
    pub fn as_variant_str(&self) -> &str
    {
        car_as_variant_str(self)
    }
    pub fn from_variant_str<T: AsRef<str>>(variantstr: T) -> Option<Self>
    {
        car_from_variant_str(variantstr)
    }
    /// Function to convert from Car to name
    pub fn as_name(&self) -> &str
    {
        car_as_name(self)
    }
    pub fn from_name(name: &str) -> Option<Self>
    {
        car_from_name(name)
    }
    /// Function to convert from Car to is_expensive
    pub fn as_is_expensive(&self) -> bool
    {
        car_as_is_expensive(self)
    }
    pub fn from_is_expensive(is_expensive: bool) -> Vec<Self>
    {
        car_from_is_expensive(is_expensive)
    }
}
impl std::fmt::Display for Car
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self {
            Self::A => writeln!(f, "{}, name = Jonda , is_expensive = false ", self.as_variant_str())?,
            Self::B => writeln!(f, "{}, name = Gord , is_expensive = false ", self.as_variant_str())?,
            Self::C => writeln!(f, "{}, name = Hamborghini , is_expensive = true ", self.as_variant_str())?,
            Self::D => writeln!(f, "{}, name = Yotota , is_expensive = true ", self.as_variant_str())?,
        }
        Ok(())
    }
}

const CAR_ALL_VARIANTS_ARRAY: [Car; 4] = [ 
    Car::A,
    Car::B,
    Car::C,
    Car::D,
];
pub const fn car_get_all_variants() -> [Car; 4]
{
    CAR_ALL_VARIANTS_ARRAY
}

// Variant string representation.
const CAR_A_STR:&'static str = "A";
const CAR_B_STR:&'static str = "B";
const CAR_C_STR:&'static str = "C";
const CAR_D_STR:&'static str = "D";

/// Returns the variants name as a &str.
pub const fn car_as_variant_str(car: &Car) -> &str
{
    match car {
        Car::A => CAR_A_STR,
        Car::B => CAR_B_STR,
        Car::C => CAR_C_STR,
        Car::D => CAR_D_STR,
    }
}

/// Returns the enum given a string that might represent the variant's name.
pub fn car_from_variant_str<T: AsRef<str>>(variantstr: T) -> Option<Car>
{
    let variantstr = variantstr.as_ref();
    match variantstr {
        CAR_A_STR => Some(Car::A),
        CAR_B_STR => Some(Car::B),
        CAR_C_STR => Some(Car::C),
        CAR_D_STR => Some(Car::D),
        _ => Option::None,
    }
}

// Constants for `name`
const NAME_A: &'static str = "Jonda";
const NAME_B: &'static str = "Gord";
const NAME_C: &'static str = "Hamborghini";
const NAME_D: &'static str = "Yotota";

/// Function to convert from Car to name
pub const fn car_as_name(car: &Car) -> &str
{
    match car {
        Car::A => NAME_A,
        Car::B => NAME_B,
        Car::C => NAME_C,
        Car::D => NAME_D,
    }
}

/// Function to convert from name to Car
pub fn car_from_name(name: &str) -> Option<Car>
{
    match name {
        NAME_A => Some(Car::A),
        NAME_B => Some(Car::B),
        NAME_C => Some(Car::C),
        NAME_D => Some(Car::D),
        _ => Option::None,
    }
}

// Constants for `is_expensive`
/// Group of 2 variants with value: `false`
const IS_EXPENSIVE_VALUE_GRP_0: [Car; 2] = [Car::A, Car::B, ];
/// Group of 2 variants with value: `true`
const IS_EXPENSIVE_VALUE_GRP_1: [Car; 2] = [Car::C, Car::D, ];

/// Function to convert from Car to is_expensive
pub const fn car_as_is_expensive(car: &Car) -> bool
{
    match car {
        Car::A => false,
        Car::B => false,
        Car::C => true,
        Car::D => true,
    }
}

/// Function to convert from is_expensive to Car
pub fn car_from_is_expensive(is_expensive: bool) -> Vec<Car>
{
    match is_expensive {
        false => IS_EXPENSIVE_VALUE_GRP_0.to_vec(),
        true => IS_EXPENSIVE_VALUE_GRP_1.to_vec(),
    }
}
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_car_do_a_barrel_roll()
    {
        let car = Car::A;
        let result = car.as_name();
        let result = Car::from_name(result).unwrap();
        let value = result.as_is_expensive();
        let vresult = Car::from_is_expensive(value);
        let result: Vec<&Car> = vresult.iter().filter_map(|x| {
            match x {
                Car::A => Some(x),
                _ => None,
            }
        }
        ).collect();
        let result = result[0].clone();
        assert_eq!(value, result.as_is_expensive());
        assert_eq!(car, result);
    }
}

fn main() {}