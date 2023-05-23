from flask import Flask
from flask_restful import Api

from .rest import Entry, EntryList


def create_app():
    app = Flask(__name__, static_folder='static')
    api = Api(app)

    @app.route('/')
    def index():
        return app.send_static_file('index.html')

    @app.route('/map')
    def map():
        return "cool folium map here"

    api.add_resource(EntryList, '/entry/')
    api.add_resource(Entry, '/entry/<entry_id>')

    return app
