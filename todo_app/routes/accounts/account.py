from todo_app.utils import RequestHandler, to_dict


class AccountsMe(RequestHandler[None, None, None, None, None], require_auth=True):
    async def get(self):
        user = await self.db.query_single("select User { id, name, todo_count := count(.todo) } filter .id = <uuid>$user_id", user_id=self.user_id)
        print(user)
        self.finish({"id": str(user.id), "name": user.name, "todo_count": user.todo_count})

route = ("/api/accounts/@me", AccountsMe)
