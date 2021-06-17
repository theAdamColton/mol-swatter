# mol-swatter


mol-swatter is a python lib that uses rust nom to quickly parse through .mol chemical files, build adjacency graphs for the molecule, and match against various functional groups hard coded into the program. Molecules and functional groups are modeled as unidirectional graphs, a molecule contains a functional group if it contains a subgraph that is isomorphic to the functional group. 

mol-swatter also contains .jdx spectra parsing functionality. In the python lib, the custom Spectrum object can be read from valid .jdx files. Spectrum exposes a handy ```transform()``` function, which can be used to make a new spectrum with specified start, end, and data points. ```transform()``` uses the average value between points to generalize when decreasing the number of data points. ```transform()``` is currently hardcoded to pad with -1's in case of lossfull transforms. 

A demonstration of parsing various .mol and .jdx files can be found in ```examples/```. You will have to scrape some .jdx and .mol files before you run the example script. 

In future iterations it would be useful to pipe the output of the parser into a tensorflow tensor, instead of rust Vectors. This could make this tool useful for machine learning on large chemical datasets utilizing .mol files, as a way to quickly convert the .mol files into a comprehensive graph representation, and the .jdx spectra into a reduced dimensional vector.

## How to build and test


- Clone this repo

- Make sure you have rust nightly installed and activated in the current directory

- Run ```./build.sh``` to build and copy the binary to the examples/ dir

- The python script ```example_imports.py``` should now be good to run. This file demonstrates how to import and use the compiled binary. If you want to run the script with lots of files, run the scraper first.



# webbook.nist.gov scraper


Uses Beautiful soup to scrape .mol and IR spectrum .sdf files from the [nist.gov](https://webbook.nist.gov/cgi/cbook.cgi?Value=10%2C1&VType=MW&Formula=&) website. Scrapes from all molecules containing carbon starting with a molar mass of 1, to as high a mass as the dataset goes. The scaper will automatically download scraped files into ```scraper/raw_data/```. Currently the scraper will not resume from where it is interrupted. You can manually change the starting molar mass by editing the URL on line 11, changing the ```......Value=10%2C*....```, where ```*``` is the desired starting molar mass.

