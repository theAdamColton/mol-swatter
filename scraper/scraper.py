import requests
from bs4 import BeautifulSoup
from urllib.parse import urlparse, urljoin
from pathvalidate import sanitize_filename
import time
import logging
import os

DATA_DIR = 'raw_data/'
LOG_DIR = 'logging/'
URL ="https://webbook.nist.gov/cgi/cbook.cgi?Value=10%2C1&VType=MW&Formula=&AllowOther=on&AllowExtra=on&Units=SI&cIR=on"

LOGGING = True
REPEAT_DL = False

def main():
    startLogger() 

    logging.info("scraping from url:"+URL)
    scrape(URL, 300) 
    
def startLogger():
    if not LOGGING:
        return

    localTime = time.localtime(time.time())
    logFileName = str(localTime.tm_mon) + "-" + str(localTime.tm_mday) + "-" + str(localTime.tm_year)[2:] + "--" \
        + str(localTime.tm_hour) + "-" + str(localTime.tm_min) + "-" + str(localTime.tm_sec)
    logFileName += ".log"

    logFilePath = uniquify(LOG_DIR + logFileName)
    logging.basicConfig(filename=logFilePath, encoding='utf-8', level=logging.INFO)


def scrape(url, maxSize):
    while True:
        try:
            res = requests.get(url)
        except Exception as e:
            logging.warning("failed to get url : " + url + " - error : " + str(e))
            break

        soup = BeautifulSoup(res.content, 'html.parser')
        
        listParent = soup.find("body").find("ol")
        
        liList = listParent.find_all('li')
        finalSize = liList[-1]

        try:
            finalSize = int(float(finalSize.find("strong").text.strip()))
        except Exception as e:
            logging.warning("failed to find finalSize :" + str(e))
            return
        
        logging.info("finalSize set to : " + str(finalSize))
        
        try:
            valueIndex = url.rfind("Value")
            url = url[:valueIndex + 6] + str(finalSize) + "%2C" + str(maxSize) \
                + "&VType=MW&Formula=&AllowOther=on&AllowExtra=on&Units=SI&cIR=on"
                                                
        except Exception as e:
            logging.warning("failed to adjust url  :" + str())
            
        parseSearchBody(soup)
        
        logging.info("new search page generated url : " + url)

def parseSearchBody(soup):
    
    listParent = soup.find("body").find("ol")
    liList = listParent.find_all('li')
    
    for li in liList:
        href = li.find("a")["href"]
        url = urljoin(URL, href)
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

    irLink = soup.select_one("a[href*=IR-SPEC]")
    
    if irLink is None:
        logging.warning("no IR-SPEC href found! link:" + url)
        return
    
    irLink = urljoin(URL, irLink['href'])

    parse_IR_page(irLink)

def parse_IR_page(url):
    try:
        res = requests.get(url)
    except Exception as e:
        logging.warning(e)
        return
    soup = BeautifulSoup(res.content, 'html.parser')
    
    # for getting the link to the JCAMP download
    jcampLink = soup.select_one("a[href*=JCAMP]")
    if jcampLink is None:
        logging.warning("could not find JCAMP href! link:" + url)
        return
    
    molFileLink = soup.select_one("a[href*=Str2File]")
    if molFileLink is not None:
        molFileLink = urljoin(URL, molFileLink['href'])

    jcampLink = urljoin(URL, jcampLink['href'])

    # For setting the file name
    chemName = soup.select_one("h1[id=Top]").text.strip() + ".jdx"
    
    jdxName = chemName.format(chemName, sanitize_filename(chemName ))
    molName = jdxName.removesuffix(".jdx") + ".mol"



    try:
        download_file(jcampLink, jdxName)

        if molName is not None:
            download_file(molFileLink, molName)
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
