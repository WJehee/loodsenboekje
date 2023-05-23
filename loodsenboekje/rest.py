from flask_restful import Resource
from datetime import datetime


class EntryItem():
    def __init__(self, entry_id, description, collaborators, location, timestamp):
        self.entry_id = entry_id
        self.description = description
        self.collaborators = collaborators
        self.location = location
        self.timestamp = timestamp
        # self.timestamp = datetime.now()


class Entry(Resource):
    def get(self, entry_id):
        return {
            'entry_id': entry_id
        }

    def put(self, entry_id):
        return ''

    def delete(self, entry_id):
        return '', 204


class EntryList(Resource):
    def get(self):
        return [
                {
                    'entry_id': 0,
                    'description': 'Bier opener',
                    'collaborators': ['Opa dorus', 'Oma dorus'],
                    'timestamp': datetime.now().strftime('%Y-%m-%d %H:%M:%S')
                }, {
                    'entry_id': 1,
                    'description': '360 bottle flip',
                    'collaborators': ['Opa dorus'],
                    'timestamp': datetime.now().strftime('%Y-%m-%d %H:%M:%S')
                }

            ]

    def post(self):
        # get latest id, increment by one
        # parse fields
        # insert and return the obj
        return {}, 201
