import csv
import requests

mapping = {
    "Pietertje": "Wouter"
}

with open('loodsenboekje.csv') as file:
    reader = csv.DictReader(file)
    for row in reader:
        how = row['wat'].strip().capitalize()
        who = [
            # person.capitalize()
            mapping.get(person.capitalize(), person.capitalize())
            for person in row['wie'].strip().split('/')]
        r = requests.post(
            'http://localhost:5000/entry/',
            json={
                "how": how,
                "who": who
            },
            headers={'Content-Type': 'application/json'}
        )
        print(r)
