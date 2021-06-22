import requests
from bs4 import BeautifulSoup
from urllib.parse import urlparse, urljoin
from pathvalidate import sanitize_filename
import time
import logging
import os
import sys

DATA_DIR = 'raw_data/'
LOGGING_PROGRESS_FILE = 'logging/progress'

# This will let the script save the last max_size for easy resuming
LOGGING = True
# Set this to true to make repeat files redownload
REPEAT_DL = False

def main():
    logging.getLogger().setLevel(logging.INFO)
    starting_mass = 1
    if len(sys.argv) > 1:
        starting_mass = int(sys.argv[1])
    else:
        res = get_progress()
        starting_mass = res if res is not None else starting_mass
    max_mass = 800
    url = get_url(starting_mass, 800)
    logging.info("Scraping from starting mass {} url: {}".format(starting_mass, url))
    scrape(url, 800) 

def get_url(starting_mass, max_mass):
    url ="https://webbook.nist.gov/cgi/cbook.cgi?Value={}-{}&VType=MW&Formula=&AllowOther=on&AllowExtra=on&Units=SI&cIR=on".format(starting_mass, max_mass)
    return url

def save_progress(mass):
    f = open(LOGGING_PROGRESS_FILE, "w")
    f.write(str(mass))
    f.close()

def get_progress():
    try:
        f = open(LOGGING_PROGRESS_FILE, "r")
        max_size = float(f.read())
        f.close()
        logging.info("Loaded {} from progress file".format(max_size))
        return max_size
    except Exception as e:
        return None

def scrape(url, max_mass):
    try:
        res = requests.get(url)
    except Exception as e:
        logging.warning("failed to get url : " + url + " - error : " + str(e))

    soup = BeautifulSoup(res.content, 'html.parser')
    list_parent = soup.find("body").find("ol")
    li_list = list_parent.find_all('li')
    # Attempts to find the molar mass of the last element in the ol list
    final_mass = li_list[-1]
    try:
        final_mass = float(final_mass.find("strong").text.strip())
    except Exception as e:
        logging.warning("failed to find finalSize :" + str(e))
        return
    logging.info("Final_size set to : " + str(final_mass))
    parseSearchBody(soup, url)
    # Once this search page has been fully scraped, saves max_size
    save_progress(final_mass)
    scrape(get_url(final_mass, max_mass), max_mass)

def parseSearchBody(soup, url):
    listParent = soup.find("body").find("ol")
    liList = listParent.find_all('li')
    for li in liList:
        href = li.find("a")["href"]
        url = urljoin(url, href)
        parseChemPage(url)

def parseChemPage(url):
    try:
        res = requests.get(url)
    except Exception as e:
        logging.warning(e)
        return
    soup = BeautifulSoup(res.content, 'html.parser')
    # if the page already has the download jdx link
    if soup.select_one("div[id=jcamp-tabs]") is not None:
        parse_IR_page(url)
        return
    # Picks out the link to the IR page
    ir_link = soup.select_one("a[href*=IR-SPEC]")
    if ir_link is None:
        logging.warning("no IR-SPEC href found! link:" + url)
        return
    ir_link = urljoin(url, ir_link['href'])
    parse_IR_page(ir_link)

def parse_IR_page(url):
    try:
        res = requests.get(url)
    except Exception as e:
        logging.warning(e)
        return
    soup = BeautifulSoup(res.content, 'html.parser')
    # for getting the link to the JCAMP download
    jcamp_link = soup.select_one("a[href*=JCAMP]")
    if jcamp_link is None:
        logging.warning("could not find JCAMP href! link:" + url)
        return
    molfile_link = soup.select_one("a[href*=Str2File]")
    if molfile_link is not None:
        molfile_link = urljoin(url, molfile_link['href'])
    jcamp_link = urljoin(url, jcamp_link['href'])
    # For setting the file name
    try:
        chemname = soup.select_one("h1[id=Top]").text.strip() + ".jdx"
        jdxname = chemname.format(chemname, sanitize_filename(chemname ))
        molname = jdxname.removesuffix(".jdx") + ".mol"
    except Exception as e:
        logging.warn("Error editing filename! {} \n\turl:{}".format(e, url))
    try:
        download_file(jcamp_link, jdxname)
        if molname is not None:
            download_file(molfile_link, molname)
    except Exception as e:
        logging.warning(e)

def download_file(url, file_name):
    try:
        r = requests.get(url, stream=True)
    except Exception as e:
        logging.warning(e)
        return
    file_path = DATA_DIR + file_name
    new_file_path = uniquify(file_path)
    if not REPEAT_DL and file_path != new_file_path:
        logging.info("repeat file:" + file_path)        
        return
    with open(file_path, 'wb') as f:
        for chunk in r.iter_content(chunk_size=1024):
            if chunk:
                f.write(chunk)
    logging.info("Downloaded " + file_path)

def uniquify(path):
    filename, extension = os.path.splitext(path)
    counter = 1
    while os.path.exists(path):
        path = filename + " (" + str(counter) + ")" + extension
        counter += 1
    return path

main()
