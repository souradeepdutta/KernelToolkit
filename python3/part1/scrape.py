import requests
import sys
from bs4 import BeautifulSoup
import re

def parse_description(raw_cve):
    description_pattern = r'Description(.*?\.)'
    match = re.search(description_pattern, raw_cve, re.DOTALL)
    if match:
        description_text = match.group(1)
        # Print the extracted description text
        print("Description:", description_text)
    else:
        print("Description not found")

def get_cve_info(cve_number):
    # Construct the URL for the NVD CVE details page
    url = f"https://nvd.nist.gov/vuln/detail/{cve_number}"
    print(f"{cve_number}")
    try:
        response = requests.get(url)

        if response.status_code == 200:
            soup = BeautifulSoup(response.content, 'html.parser')
            cve_details = soup.find("div", id="vulnDetailPanel")

            if cve_details:
                cve_details = cve_details.text.strip()
            else:
                cve_details = "Title not found"

            # Print the CVE information
            raw_cve = cve_details
            parse_description(raw_cve)
        else:
            print("Error: HTTP status code", response.status_code)

    except Exception as e:
        print("An error occurred:", str(e))

def get_cve_info2(cve_number):
    # Construct the URL for the NVD CVE details page
    url = f"https://www.cvedetails.com/cve/{cve_number}/"
    print(f"\n{cve_number} references")
    try:
        #response = requests.get(url)
        session = requests.Session()
        response = session.get(url, headers={'User-Agent': 'Mozilla/5.0'})
        if response.status_code == 200:
            soup = BeautifulSoup(response.content, 'html.parser')
            cve_references = [a['href'] for a in soup.find_all("a", class_="ssc-ext-link")]
            for reference in cve_references:
                print(reference)
            # Print the CVE information
        else:
            print("Error: HTTP status code", response.status_code)

    except Exception as e:
        print("An error occurred:", str(e))

if __name__ == "__main__":
    cve_number = "CVE-"+sys.argv[1]
    get_cve_info(cve_number)
    get_cve_info2(cve_number)
    """
    cve_number = "CVE-2022-3047"
    get_cve_info(cve_number)
    get_cve_info2(cve_number)
    """
