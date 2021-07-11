/**
 * Contains the spectrum struct and methods, which contains the pertinent information we want to parse from the jcamp file.
 **/
 
use crate::debug_println;
use crate::constants::DEBUG_LEVEL;

// Handles whether the Xunits are 1/cm or 1/um
#[derive(Copy, Clone)]
pub enum Xunits {
    cm,
    um,
}

pub struct Spectrum {
    // ex : "WATER"
    name : String,
    // ex : "INFRARED SPECTRUM"
    spectrum_type : String,
    // ex : "LIQUID (NEAT)"
    state : String,
    // xunits can ONLY be cm
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
    pub fn new(name : &str, spectrum_type : &str, state : &str, xunits : Xunits, first_x : f32, last_x : f32, npoints : i32) -> Spectrum {
        let (mut first_x, mut last_x) = (first_x, last_x);
        // Enforces that first_x is less than last_x
        if first_x > last_x {
            let temp = first_x;
            first_x = last_x;
            last_x = temp;
        }
        // Handles different xunit cases
        match xunits {
            Xunits::cm => {},
            Xunits::um => {
                let temp = first_x;
                first_x = 10000.0 / last_x;
                last_x = 10000.0 / temp;
            }
        }

        assert!(npoints > 1 );
        let delta_x : f32 = (last_x - first_x) / (npoints as f32 - 1.0);
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
        self.y_values.push(val * self.y_factor);            
    }
 
    // Fit this spectrum into a different shaped spectrum
    pub fn transform(&self, first_x : f32, last_x : f32, npoints : i32) -> Spectrum { 
        assert!(first_x < last_x);
        // This molecule must be complete
        assert!(self.is_complete());
        // Can only have positive non-zero number of npoints
        assert!(npoints > 1);
        let mut spec = Spectrum::new(
            &self.name, &self.spectrum_type, &self.state, Xunits::cm, first_x, last_x, npoints); 
        let delta_x: f32 = (last_x - first_x) / (npoints as f32);
        // Iterate curr_x from first_x to last_x by delta_x
        for i in 1..npoints + 1 {
            let curr_x : f32 = (first_x) + i as f32 * delta_x;
            let prev_x : f32 = curr_x - delta_x;
            spec.add_y(self.find_slice_average(prev_x, curr_x));
        }
        assert!(spec.is_complete());
        spec
    }

    // Returns all of the Y values
    pub fn get_y_values(&self) -> Vec<f32> {
        self.y_values.to_owned()
    }

    // Generates and returns all of the x values
    pub fn get_x_values(&self) -> Vec<f32> {
        let mut out : Vec<f32> = Vec::new();
        for count in 0..self.y_values.len() {
            out.push(count as f32 * self.delta_x + self.first_x); 
        }
        out
    }

    pub fn to_string(&self) -> String {
        format!("{}, {}\n\tData from {} to {} by {}, npoints:{}, Spectrum is complete: {}",
            self.name, self.spectrum_type, self.first_x, self.last_x, self.delta_x, self.npoints, self.is_complete())
    }

    pub fn print_xy(&self) {
        let y_values : Vec<f32> = self.get_y_values();
        let x_values : Vec<f32> = self.get_x_values();
        assert_eq!(x_values.len(), y_values.len());
        for i in 0..y_values.len() {
            print!("({}, {})", x_values[i], y_values[i]);
        }       
        println!("");
    }

    // Finds the average value of y values between two x values inclusive
    fn find_slice_average(&self, first_x : f32, last_x : f32) -> f32 {
        let mut count = 0;
        let mut sum = 0.0;
        let from_i = self.i_of(first_x);
        let to_i = self.i_of(last_x);
        for i in from_i..to_i+1 {
            count+=1;
            sum += self.y_values[i];
        }
        if count == 0 {
            return self.f_of(first_x)
        } else {
            sum / (count as f32)
        }
    }
    
    // Get the value of y at any x
    pub fn f_of(&self, x : f32) -> f32 {
        let index = self.i_of(x);
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

    // Get the nearest index of any x
    fn i_of(&self, x : f32) -> usize {
        let i = ((x - self.first_x) / self.delta_x).round() as usize;
        if i > self.npoints as usize{
            return self.npoints as usize
        }
        i
    }


    // Checks the last index of x to see if the spectrum has been fully filled with y values
    pub fn is_complete(&self) -> bool {
        if self.npoints as usize == self.y_values.len() {
            return true
        } else {
            debug_println!("Incomplete! npoints {}, y_values.len() {}", self.npoints, self.y_values.len());
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

        let mut spectrum_2 = Spectrum::new("KRYPTONITE", "WAFER SPECTRUM", "PLASMA", Xunits::cm, 1.0, 4.0, 4);
        let to_add = vec!(0.1, 0.2, 0.3, 0.4);
        for y in to_add {
            spectrum_2.add_y(y);
        }
        
        assert_eq!(spectrum_1.y_values, spectrum_2.y_values);
        assert_eq!(spectrum_1.name, spectrum_2.name);
    }
    #[test]
    fn test_private_methods() {
        let mut spectrum = Spectrum::new("Pretend molecule", "stethescope", "beam", Xunits::cm, 4.0, 8.0, 3);
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
        assert_eq!(spectrum.f_of(4.999), 1.0);
        assert_eq!(spectrum.f_of(5.0), 1.1);
        assert_eq!(spectrum.f_of(5.6), 1.1);
        assert_eq!(spectrum.f_of(6.99), 1.1);
        assert_eq!(spectrum.f_of(7.0), 1.2);
        assert_eq!(spectrum.f_of(7.1), 1.2);
        assert_eq!(spectrum.f_of(7.9), 1.2);
        assert_eq!(spectrum.f_of(8.0), 1.2);


        // calculates average manually
        let avg = (spectrum.f_of(4.0) + spectrum.f_of(6.0) + spectrum.f_of(8.0)) / 3.0;

        let mut sum =0.0;
        for y in spectrum.get_y_values() {
            sum += y;
        }
        let avg2 = sum / spectrum.get_y_values().len() as f32;

        assert_eq!(avg, 1.1);
        assert_eq!(avg, avg2);
        assert_eq!(spectrum.find_slice_average(4.0, 8.0), avg);
    }

    #[test]
    fn test_transform_spectrum() {
        let mut spectrum = Spectrum::new("Pretend molecule", "stethescope", "beam", Xunits::cm, 4.0, 8.0, 3);
        spectrum.add_y(1.0);
        spectrum.add_y(1.1);
        spectrum.add_y(1.2);
        println!("{}", spectrum.to_string());
        assert!(spectrum.is_complete());
        spectrum.print_xy();
        let res = spectrum.transform(4.0, 6.0, 2);
        println!("{}", res.to_string());
        res.print_xy();
        assert!(res.is_complete());
    }

    #[test]
    fn test_big_transform() {
        // Creates a spectrum of 388.677 to 3799.46 by 0.870985
        let spectrum = get_spectrum("Water.jdx");
        assert!(spectrum.is_complete());
        assert_eq!(spectrum.get_x_values().len(), spectrum.get_y_values().len());
        let transformed_spectrum = spectrum.transform(100.0, 1000.0, 10);
        println!("{}", transformed_spectrum.to_string());
        transformed_spectrum.print_xy();
        assert_eq!(transformed_spectrum.get_x_values().len(), 
            transformed_spectrum.get_y_values().len());
    }

    #[test]
    fn test_comprehensive_transform() {
        let spectrum = get_spectrum("Styrene, oligomers.jdx");
        for hi in (0..4000).step_by(111) {
            for lo in (0..4000).step_by(111) {
                if lo >= hi {
                    break
                }
                for res in (5..500).step_by(41) {
                    println!("lo {},hi {},res {}", lo, hi, res);
                    spectrum.transform(lo as f32, hi as f32, res);
                }
            }
        }
    }

    #[test]
    fn test_funky_files() {
        let spectrum = get_spectrum("(5Z)-3-Methyl-1,5-heptadiene.jdx");
        let trans_spec = spectrum.transform(400.0, 3000.0, 2600);
        println!("{}", trans_spec.to_string());
        let spectrum = get_spectrum("4-Octene, (Z)-.jdx");
        spectrum.transform(300.0, 4000.0, 100);
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
