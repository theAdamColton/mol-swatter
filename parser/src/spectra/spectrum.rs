/**
 * Contains the spectrum struct and methods, which contains the pertinent information we want to parse from the jcamp file.
 **/

pub struct Spectrum {
    // ex : "WATER"
    name : String,
    // ex : "INFRARED SPECTRUM"
    spectrum_type : String,
    // ex : "LIQUID (NEAT)"
    state : String,
    // X is always in units of  / cm
    // delta_x is very important, in each column of the XYDATA=(X++(Y..Y)), the Y values in each
    // column have their corresponding x values incremented by delta_x
    //
    // ex : with a delta_x of 1, and a first_x of 0 this row:
    // 0 .023 .024 .026 .030
    //
    // corresponds to these x,y points :
    // (0,.023), (1, .024), (2, .026), (3, .030)
    delta_x : f32, 
    first_x : f32,
    last_x : f32,
    // y_values is the meat of the spectrum, each point is incremented by delta_x starting from
    // first_x
    y_values : Vec<f32>,
    // number of data points
    npoints : i32,
    // multiply y values by this
    y_factor : f32,
}

impl Spectrum {
    // Instantiate a new Spectrum with empty y_values vector
    pub fn new(name : &str, spectrum_type : &str, state : &str, first_x : f32, last_x : f32, npoints : i32) -> Spectrum {

        let (mut first_x, mut last_x) = (first_x, last_x);
        // Enforces that first_x is less than last_x
        if first_x > last_x {
            let temp = first_x;
            first_x = last_x;
            last_x = temp;
        }

        let delta_x = (last_x - first_x) / (npoints as f32);
        Spectrum {
            name : name.to_string(),
            spectrum_type : spectrum_type.to_string(),
            state : state.to_string(),
            delta_x,
            npoints,
            first_x,
            last_x,
            y_factor : 1.0,
            y_values : Vec::new()
        }
    }

    pub fn set_y_factor(&mut self, y_factor : f32) {
        self.y_factor = y_factor;
    }
    
    // Add a single y value to the Spectra
    pub fn add_y(&mut self, val : f32) {
        &self.y_values.push(val * self.y_factor);            
    }

    // Fit this spectrum into a different shaped spectrum
    pub fn transform(&self, first_x : f32, last_x : f32, npoints : i32) -> Spectrum { 

        assert!(first_x < last_x);
        // This molecule must be complete
        assert!(self.is_complete());

        let delta_x : f32 = (last_x - first_x) / (npoints as f32);

        let mut spec = Spectrum::new(
            &self.name, &self.spectrum_type, &self.state, first_x, last_x, npoints); 

        // Iterate curr_x from first_x to last_x by delta_x
        let mut curr_x = first_x + delta_x;
        let mut prev_x = first_x;
        loop {
            if curr_x > last_x {
                break
            }

            spec.add_y(self.find_slice_average(prev_x, curr_x));
            prev_x = curr_x;
            curr_x += delta_x;
        }
        spec
    }

    // Returns all of the Y values
    pub fn get_y_values(&self) -> Vec<f32> {
        assert!(self.is_complete());
        self.y_values.to_owned()
    }

    // Generates and returns all of the x values
    pub fn get_x_values(&self) -> Vec<f32> {
        let mut out : Vec<f32> = Vec::new();
        let mut curr_x = self.first_x;
        while curr_x <= self.last_x {
            out.push(curr_x);
            curr_x += self.delta_x;
        }
        out
    }

    pub fn to_string(&self) -> String {
        format!("{}, {}\n\tData from {} to {} by {}, npoints:{}, Spectrum is complete: {}",
            self.name, self.spectrum_type, self.first_x, self.last_x, self.delta_x, self.npoints, self.is_complete())
    }

    pub fn print_xy(&self) {
        assert!(self.is_complete());
        let y_values : Vec<f32> = self.get_y_values();
        let x_values : Vec<f32> = self.get_x_values();
        for i in 0..y_values.len() {
            print!("({}, {})", x_values[i], y_values[i]);
        }       
        println!("");
    }

    // Finds the average value of y values between two x values inclusive
    fn find_slice_average(&self, first_x : f32, last_x : f32) -> f32 {
        //assert!(first_x >= self.first_x);
        //assert!(last_x <= self.last_x);
        let mut curr_x = first_x;
        let mut count = 0.0;
        let mut sum = 0.0;
        loop {
            if curr_x >= last_x {
                break
            }
            count +=1.0;
            sum += self.f_of(curr_x);
            curr_x += self.delta_x;
        }
        if count == 0.0 {
            return self.f_of(first_x)
        } else {
            sum / count
        }
    }

    // Get the value of y at any x
    fn f_of(&self, x : f32) -> f32 {
        let index : usize = ((x - self.first_x) / self.delta_x).round() as usize;

        match self.y_values.get(index) {
            Some(x) => {
                return x.to_owned()
            }
            // Pads with -1s
            None => {
                return -1.0
            }
        }
    }

    // Checks the last index of x to see if the spectrum has been fully filled with y values
    pub fn is_complete(&self) -> bool {
        if self.npoints as usize == self.y_values.len(){
            return true
        } else {
            return false
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use super::super::parser;
    use crate::constants::TEST_DIR;
    #[test]
    fn test_new_spectrum() {
        let spectrum_1 = Spectrum {
            name : "KRYPTONITE".to_string(),
            spectrum_type : "WAFER SPECTRUM".to_string(),
            state : "PLASMA".to_string(),
            delta_x : 1.0,
            npoints : 4,
            first_x : 100.0,
            last_x : 103.0,
            y_factor : 1.0,
            y_values : vec![0.1, 0.2, 0.3, 0.4]
        };

        let mut spectrum_2 = Spectrum::new("KRYPTONITE", "WAFER SPECTRUM", "PLASMA", 1.0, 4.0, 4);
        let to_add = vec!(0.1, 0.2, 0.3, 0.4);
        for y in to_add {
            spectrum_2.add_y(y);
        }
        
        assert_eq!(spectrum_1.y_values, spectrum_2.y_values);
        assert_eq!(spectrum_1.name, spectrum_2.name);
    }
    #[test]
    fn test_private_methods() {
        let mut spectrum = Spectrum::new("Pretend molecule", "stethescope", "beam", 4.0, 8.0, 3);
        assert_eq!(spectrum.is_complete(), false);
        spectrum.add_y(1.0);
        assert_eq!(spectrum.is_complete(), false);
        spectrum.add_y(1.1);
        assert_eq!(spectrum.is_complete(), false);
        spectrum.add_y(1.2);
        assert!(spectrum.is_complete());

        println!("{}", spectrum.to_string());
        spectrum.print_xy();
        
        println!("{}", spectrum.to_string());
        assert_eq!(spectrum.f_of(4.0), 1.0);
        assert_eq!(spectrum.f_of(4.1), 1.0);
        assert_eq!(spectrum.f_of(4.4), 1.0);
        assert_eq!(spectrum.f_of(5.4), 1.1);
        assert_eq!(spectrum.f_of(5.6), 1.1);
        assert_eq!(spectrum.f_of(5.99), 1.1);
        assert_eq!(spectrum.f_of(6.1), 1.2);
        assert_eq!(spectrum.f_of(6.4), 1.2);

        // calculates average manually
        let mut sum = spectrum.f_of(4.0);
        sum += spectrum.f_of(5.333);
        sum += spectrum.f_of(6.6666667);
        let avg = sum / 3.0;
        
        assert_eq!(avg, 1.1);
        assert_eq!(spectrum.find_slice_average(4.0, 8.0), avg);
    }
    #[test]
    fn test_transform_spectrum() {
        let mut spectrum = Spectrum::new("Pretend molecule", "stethescope", "beam", 4.0, 8.0, 3);
        spectrum.add_y(1.0);
        spectrum.add_y(1.1);
        spectrum.add_y(1.2);

        println!("{}", spectrum.to_string());
        spectrum.print_xy();
        
        let res = spectrum.transform(4.0, 6.0, 1);
        
        println!("{}", res.to_string());
        res.print_xy();

        assert!(res.is_complete());
    }
    #[test]
    fn test_big_transform() {
        // Creates a spectrum of 388.677 to 3799.46 by 0.870985
        let spectrum = get_spectrum("Water.jdx");
        // Transforms into new spectrum of 400.0 to 3000.0 by 1.0
        let transformed_spectrum = spectrum.transform(400.0, 3000.0, 2600);
        println!("{}", transformed_spectrum.to_string());
        transformed_spectrum.print_xy();
    }
    #[test]
    fn test_funky_files() {
        let spectrum = get_spectrum("(5Z)-3-Methyl-1,5-heptadiene.jdx");
        let trans_spec = spectrum.transform(400.0, 3000.0, 2600);
        println!("{}", trans_spec.to_string());
        let spectrum = get_spectrum("4-Octene, (Z)-.jdx");
    }
    #[test]
    fn test_funky_file_2() {
        let spec = get_spectrum("Benzeneacetamide, «alpha»-ethyl-.jdx");
        let trans_spec = spec.transform(500.0, 3000.0, 2300);        
        println!("{}", trans_spec.to_string());
    }
    fn get_spectrum(file : &str) -> Spectrum {
        let spectrum : Spectrum = parser::parse_jdx(&(TEST_DIR.to_owned() + file)).unwrap();
        println!("{}", spectrum.to_string());
        spectrum
    }
}
