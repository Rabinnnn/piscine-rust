/*
Define a data structure called light that has 2 fields: alias and brightness.
An associated function new creating a new light with the alias passed an argument and the brightness of 0.
A function 'change_brightness' receiving a mutable vector of lights, an alias and a brightness with u8 value. It will find the light in the vec with the given alias and set change to the brightness0
*/

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        Self {
            alias: alias.to_string(),
            brightness: 0,
        }
    }
}
/*
The function change_brightness starts by calling the iter_mut method on lights vector
This method returns a mutable iterator over the vector, meaning you can modify the elements of the vector(alias or brightness)
Next, the find method is called on the iterator. THis find method takes a closer that returns true if the alias of the Light instance
matches the provided alias. If a match is found, 'find' returns 'Some(light)' where light is a mutable reference to the Light instance.
if no match is found, 'find' returns 'None'.

The 'if let' statement is used to check if 'Some(light)' is returned. If so, it sets the brightness of the Light instance to the provided brightness.
If 'None' is returned, nothing happens because the if let statement will not executed
*/
pub fn change_brightness(lights: &mut Vec<Light>, alias: &str, brightness: u8) {
    if let Some(light) = lights.iter_mut().find(|light| light.alias == alias) {
        light.brightness = brightness;
    }
}