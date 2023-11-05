import requests
from bs4 import BeautifulSoup
import subprocess

result = subprocess.run(["uname", "-r"], stdout=subprocess.PIPE, text=True)
kernel_version = result.stdout.strip()
print("Kernel Version:"+kernel_version)

url = "https://nvd.nist.gov/vuln/search/results?form_type=Basic&results_type=overview&query="+kernel_version

response = requests.get(url)

if response.status_code == 200:
    soup = BeautifulSoup(response.content, 'html.parser')
    
    table = soup.find('table', {'class': 'table table-striped table-hover'})
    
    if table:
        for row in table.find_all('tr'):
            columns = row.find_all('td')
    else:
        print("Table not found on the page.")

    for i in range(3): 
        vulnerability_summary = soup.find('p', {'data-testid': f'vuln-summary-{i}'}).text
        published_date = soup.find('span', {'data-testid': f'vuln-published-on-{i}'}).text

        print("Vulnerability Summary:", vulnerability_summary)
        print("Published Date:", published_date)
        i+=1
        print()
else:
    print(f"Failed to retrieve the page. Status code: {response.status_code}")


