
# Python Scaper for [NIST Chemistry WebBook, SRD 69](https://webbook.nist.gov/cgi/cbook.cgi?Value=1%2C130&VType=MW&Formula=&AllowExtra=on&Units=SI&cIR=on)

## How to Use Scraper

- It is a good idea to first make a new virtual env

- Install requirements; from this dir ```pip install -r requirements.txt```

- The scraper's default behavior is to start scraping from a starting molar mass of 1. You may pass a single argument to the script to change the starting mass. Ex: ```python scraper.py 100```

- The script will save the progress of the scraper by storing the molar mass in logging/progress. 
