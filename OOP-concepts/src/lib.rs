// Rust is object oriented: structs and enums have data, and impl blocks provide methods on structs and enums.
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64, // fields remain private
}

// Encapsulation discussed in Ch.7
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    // private method (hide implementation details)
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    // Inheritance: For reuse of code (Discussed and available via Traits)
}

/*
 An example of a GUI tool, where each component will need to 'implement' the Draw method
*/

// Trait objects aren’t as generally useful as objects in other languages:
//  their specific purpose is to allow abstraction across common behavior.
pub trait Draw {
    fn draw(&self);
}

// holds components: a vector of trait objects
// similar to 'extending' an object
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        // This works differently from defining a struct that uses a generic type parameter with trait bounds.
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Rendering button.")
    }
}

/*
  Using trait bounds would limit us to only one type of component, i.e. Button

  pub struct Screen<T: Draw> {
      pub components: Vec<T>,
  }

  impl<T> Screen<T>
  where
      T: Draw,
  {
      pub fn run(&self) {
          for component in self.components.iter() {
              component.draw();
          }
      }
  }
*/
