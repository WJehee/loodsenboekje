from flask import Flask
from flask_restful import Api

from .routes import Entry, bp


def create_app():
    app = Flask(__name__)
    api = Api(app)

    app.register_blueprint(bp)
    api.add_resource(Entry, '/entry/<entry_id>')

    return app
