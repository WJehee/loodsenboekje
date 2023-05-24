import os

from flask import Flask
from flask_restful import Api

from .rest import Entry, EntryList
from . import db, routes


def create_app():
    app = Flask(__name__, static_folder='static', instance_relative_config=True)
    jinja_options = app.jinja_options
    jinja_options.update({
        'variable_start_string': '(%',
        'variable_end_string': '%)',
    })
    app.jinja_options = jinja_options

    app.config.from_mapping(
        SECRET_KEY='dev',
        DATABASE=os.path.join(app.instance_path, 'loodsenboekje.sqlite'),
        SESSION_COOKIE_HTTPONLY=True,
        REMEMBER_COOKIE_HTTPONLY=True,
        SESSION_COOKIE_SAMESITE="Strict",
    )
    try:
        os.makedirs(app.instance_path)
    except OSError:
        pass

    api = Api(app)
    db.init_app(app)
    routes.init_app(app)

    app.register_blueprint(routes.bp)

    api.add_resource(EntryList, '/entry/')
    api.add_resource(Entry, '/entry/<entry_id>')

    return app
