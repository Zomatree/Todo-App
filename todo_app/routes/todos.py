from typing import TypedDict

from todo_app.utils import RequestHandler


class PostBody(TypedDict):
    title: str

class Todos(RequestHandler[None, PostBody, None, None, None], require_auth=True):
    async def post(self):
        title = self.post_body["title"]

        user = await self.db.query_single("""
            update User
            filter .id = <uuid>$user_id
            set {
                todo += (
                    insert Todo {
                        title := <str>$title
                    }
                )
            }
        """, title=title, id=id, user_id=self.user_id)
        print(user.todo[0])

        self.set_status(201)
        self.finish({"title": title, "todo_id": "0"})

    async def get(self):
        todos = await self.db.query_json("""
            select User {
                todo: {
                    title,
                    id,
                    completed,
                    created_at,
                    description
                }
            }
            filter .id = <uuid>$user_id
        """, user_id=self.user_id)

        self.finish(todos[1:-1])  # its a list of dicts so im removing the `[]` chars manually because its a json string

route = ("/api/todos", Todos)
