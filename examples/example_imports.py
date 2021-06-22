# This python file demonstrates importing and basic use of the rust mol_getter lib
import mol_swatter
import os
import time
import matplotlib.pyplot as plt

DATA_DIR = "../scraper/raw_data/"


def demonstrate_spectrum_parser(file_path):
    spectrum = mol_swatter.Spectrum(file_path)
    if not spectrum.is_valid():
        print("\tINVALID FILE :" + file_path)
        return
    print(spectrum.to_string())
    new_spectrum = spectrum.transform(800, 3000, 1000)
    print(new_spectrum.to_string())


def iterate_over_files(file_extension, funct):
    cntr = 0
    avr_time = 0
    longest_time = 0
    longest_time_mol = ""

    for filename in os.listdir(DATA_DIR):
        if filename.endswith(file_extension):
            file_path = os.path.join(DATA_DIR, filename)
            print("\n***********************************")
            print(file_path)
            start = time.time() * 1000

            funct(file_path)

            cntr += 1

            end = time.time()*1000 - start
            if end > longest_time:
                longest_time = end
                longest_time_mol = filename
            print("*************{} ms***************".format(end))
            avr_time += end

    avr_time = avr_time / cntr
    print("\n")
    print("Processed {} files at an average of {} ms per file".format(cntr, avr_time))
    print("Longest file was {} at {} ms".format(longest_time_mol, longest_time))

# ----- Mol Parser Demonstration -----
input("Press enter to continue, The script will parse through all .mol files in the DATA_DIR")
# Requires mol_swatter.so in the same directory
myParser = mol_swatter.ParseGroups()

# Prints out the list of functional groups mol_getter is using
functional_groups = myParser.get_funct_groups()
print("Functional groups : \n" + str(functional_groups))

# Prints the rust internal object result, returns the boolean list to result
# This requires the scraper to have downloaded Pentanoic acid.mol, otherwise you will not have this file.
print("\nStyrene functional groups result:")
myParser.get_funct_result_and_print(DATA_DIR + "Styrene, oligomers.mol")

# Bulk process of the mol files in DATA_DIR
iterate_over_files(".mol", myParser.get_funct_result_and_print)

# ----- Spectrum Parser Demonstration -----
input(
    """Press enter to continue, the script will
show the data transform methods of the .jdx spectra""")
spectrum = mol_swatter.Spectrum(DATA_DIR + "Water.jdx")
print(spectrum.to_string())
x = spectrum.get_x_values()
y = spectrum.get_y_values()
plt.plot(x, y)
ax = plt.gca()
ax.set_xlim(ax.get_xlim()[::-1])
plt.show()
input(
    """Press enter to continue, the script will 
parse through all of the .jdx spectra in the DATA_DIR and 
perform a single transformation on each.""")

iterate_over_files(".jdx", demonstrate_spectrum_parser)
