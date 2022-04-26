from typing import TypedDict
from typing_extensions import NotRequired

from todo_app.utils import RequestHandler

class PatchBody(TypedDict):
    title: NotRequired[str]
    description: NotRequired[str]
    complete: NotRequired[bool]

class EditTodos(RequestHandler[None, None, None, PatchBody, None], require_auth=True):
    async def patch(self, todo_id_param: str):
        try:
            todo_id = int(todo_id_param)
        except ValueError:
            return self.send_error(400, reason="todo id must be an integer")

        new_title = self.patch_body.get("title", None)
        new_description = self.patch_body.get("description", None)
        new_complete = self.patch_body.get("complete", None)

        output = await self.db.query("""
            update Todo
            filter .todo_id = <int64>$todo_id
            set {
                title := <optional str>$title ?? .title,
                description := <optional str>$description ?? .description,
                completed := <optional bool>$complete ?? .completed
            }
        """, todo_id=todo_id, title=new_title, description=new_description, complete=new_complete)
        print(output)

        self.set_status(204)
        self.finish()

    async def delete(self, todo_id_param: str):
        try:
            todo_id = int(todo_id_param)
        except ValueError:
            return self.send_error(400, reason="todo id must be an integer")

        await self.db.query("delete Todo filter .todo_id = <int64>$todo_id", todo_id=todo_id)

        self.set_status(204)
        self.finish()

route = ("/api/todos/(.+)", EditTodos)
