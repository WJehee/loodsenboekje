import os

from flask import Flask
from flask_restful import Api

from .rest import Entry, EntryList
from .db import init_app


def create_app():
    app = Flask(__name__, static_folder='static', instance_relative_config=True)
    app.config.from_mapping(
            SECRET_KEY='dev',
            DATABASE=os.path.join(app.instance_path, 'loodsenboekje.sqlite'),
    )
    try:
        os.makedirs(app.instance_path)
    except OSError:
        pass
    api = Api(app)
    init_app(app)

    @app.route('/')
    def index():
        return app.send_static_file('index.html')

    @app.route('/map')
    def map():
        return "cool folium map here"

    api.add_resource(EntryList, '/entry/')
    api.add_resource(Entry, '/entry/<entry_id>')

    return app
