from flask import Blueprint, render_template
from flask_restful import Resource
from datetime import datetime


bp = Blueprint('routes', __name__)


@bp.route('/')
def index():
    return render_template('index.html')


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


class EntryList(Resource):
    def get(self):
        # return list of all entries
        return

    def post(self):
        # get latest id, increment by one
        # parse fields
        # insert and return the obj
        return {}, 201

