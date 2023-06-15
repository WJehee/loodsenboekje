from datetime import datetime
from flask_restful import Resource, reqparse
from flask_login import login_required

from loodsenboekje.db import get_db
from loodsenboekje.routes import check_authenticated


parser = reqparse.RequestParser()
parser.add_argument('how', type=str, required=True)
parser.add_argument('who', required=True, action='append')


class Entry(Resource):
    def get(self, entry_id):
        db = get_db()
        entry = db.execute(
            "SELECT * FROM entry WHERE id = ?",
            (entry_id,)
        ).fetchone()
        return {
            'id': entry['id'],
            'how': entry['how'],
            'created': entry['created'].strftime("%d/%m/%Y"),
        }

    def put(self, entry_id):
        return ''

    def delete(self, entry_id):
        db = get_db()
        db.execute(
            "DELETE FROM entry WHERE id = ?",
            (entry_id,)
        )
        db.commit()
        return '', 204


class EntryList(Resource):
    def get(self):
        db = get_db()
        entries = db.execute(
            """
            SELECT entry.id, entry.how, GROUP_CONCAT(user.name) as who,
            entry.created FROM user
            JOIN entry_user ON user.id = entry_user.user_id
            JOIN entry ON entry.id = entry_user.entry_id
            GROUP BY entry.id
            """
        ).fetchall()
        return [{
            'id': entry["id"],
            'how': entry["how"],
            'who': entry["who"].split(','),
            'created': entry["created"].strftime("%d/%m/%Y")
        } for entry in entries]

    @check_authenticated
    def post(self):
        args = parser.parse_args()
        db = get_db()
        cur = db.cursor()
        # Insert new entry
        cur.execute(
            "INSERT OR IGNORE INTO entry (how) VALUES (?)",
            (args["how"],)
        )
        entry_id = cur.lastrowid
        # If already in database, return early
        if entry_id == 0:
            return '', 400
        # Insert new persons if they are not already in the database
        person_ids = []
        for person in args["who"]:
            cur.execute(
                "INSERT OR IGNORE INTO user (name) VALUES (?)",
                (person,)
            )
            person_ids.append(cur.execute(
                "SELECT * FROM user WHERE name = ?",
                (person,)
            ).fetchone()['id'])
        # Add entry to the join table
        for person_id in person_ids:
            cur.execute(
                "INSERT INTO entry_user (entry_id, user_id) VALUES (?, ?)",
                (entry_id, person_id)
            )
        db.commit()
        res = {
            'id': entry_id,
            'created': datetime.now().strftime("%d/%m/%Y")
        }
        res.update(args)

        return res, 201
