# This python file demonstrates importing and basic use of the rust mol_getter lib
import mol_swatter
import os
import time


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



## Mol Parser Demonstration

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

input("Press enter to continue")

## Spectrum Parser Demonstration
iterate_over_files(".jdx", demonstrate_spectrum_parser)
