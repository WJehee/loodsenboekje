from dotenv import dotenv_values
from functools import wraps
from flask import Blueprint, render_template, request, url_for, redirect
from flask_login import login_required, UserMixin, login_user, logout_user, LoginManager, current_user
from flask_httpauth import HTTPBasicAuth
from werkzeug.exceptions import Unauthorized


class User(UserMixin):
    pass


basic_auth = HTTPBasicAuth()
users = {
    'Loods': {
        'password': dotenv_values()['PASSWORD'],
    }
}

bp = Blueprint('routes', __name__)


def check_authenticated(func):
    @wraps(func)
    def decorated(*args, **kwargs):
        if current_user.is_authenticated:
            return func(*args, **kwargs)

        auth = request.authorization
        if auth and auth.username in users and auth.password == users[auth.username]['password']:
            return func(*args, **kwargs)

        raise Unauthorized()
    return decorated


def init_app(app):
    login_manager = LoginManager()
    login_manager.init_app(app)

    @login_manager.user_loader
    def user_loader(username):
        if username not in users:
            return
        user = User()
        user.id = username
        return user

    @login_manager.request_loader
    def request_loader(request):
        username = request.form.get('username')
        if username not in users:
            return
        user = User()
        user.id = username
        return user


@bp.route('/')
def index():
    return render_template('index.html')


@bp.route('/login', methods=['GET', 'POST'])
def login():
    if request.method == 'GET':
        return render_template('login.html')
    username = request.form['username']

    if username in users and request.form['password'] == users[username]['password']:
        user = User()
        user.id = username
        login_user(user)
        return redirect(url_for('routes.index'))
    # TODO: Flash a message about the error
    return render_template('login.html')


@bp.route("/logout")
@login_required
def logout():
    logout_user()
    return redirect(url_for('routes.index'))
