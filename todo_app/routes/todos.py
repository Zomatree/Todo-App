from typing import TypedDict
from typing_extensions import NotRequired

from todo_app.utils import RequestHandler, create_id


class PostBody(TypedDict):
    title: str

class Todos(RequestHandler[None, PostBody, None, None, None], require_auth=True):
    async def post(self):
        title = self.post_body["title"]
        id = create_id()

        await self.db.query("""
            update User
            filter .user_id = <int64>$user_id
            set {
                todo += (
                    insert Todo {
                        title := <str>$title,
                        todo_id := <int64>$id,
                    }
                )
            }
        """, title=title, id=id, user_id=self.user_id)

        self.set_status(201)
        self.finish({"title": title, "todo_id": id})

    async def get(self):
        todos = await self.db.query_json("""
            select User {
                todo: {
                    title,
                    todo_id,
                    completed,
                    created_at,
                    description
                }
            }
            filter .user_id = <int64>$user_id
        """, user_id=self.user_id)

        self.finish(todos[1:-1])  # its a list of dicts so im removing the `[]` chars manually because its a json string

route = ("/api/todos", Todos)
