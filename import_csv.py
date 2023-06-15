import csv
import sys
import requests

mapping = {
    "Pietertje": "Wouter"
}

if len(sys.argv) <= 2:
    print("Not enough arguments, need username + password")
    exit(1)

session = requests.Session()
session.auth = (sys.argv[1], sys.argv[2])

with open('loodsenboekje.csv') as file:
    reader = csv.DictReader(file)
    for row in reader:
        how = row['wat'].strip().capitalize()
        who = [
            mapping.get(person.capitalize(), person.capitalize())
            for person in row['wie'].strip().split('/')]
        try:
            r = session.post(
                'http://localhost:5000/entry/',
                json={
                    "how": how,
                    "who": who
                },
                headers={'Content-Type': 'application/json'}
            )
            print(r)
        except requests.exceptions.ConnectionError:
            print("Connection refused, try a different username / password")
            exit(1)
