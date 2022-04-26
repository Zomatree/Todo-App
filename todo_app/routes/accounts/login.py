from typing import TypedDict

from todo_app.utils import RequestHandler


class PostBody(TypedDict):
    username: str
    password: str

class Login(RequestHandler[None, PostBody, None, None, None]):
    async def post(self):
        name = self.post_body["username"]

        query = await self.db.query("""
            select User {
                name,
                user_id,
                password
            }
            filter .name = <str>$name
        """, name=name)

        if not query:
            return self.send_error(400, reason="incorrect login info")

        user = query[0]

        if not self.password_hasher.verify(self.post_body["password"], user.password):
            return self.send_error(400, reason="incorrect login info")

        token = self.tokens.create_token(user.user_id)

        self.finish({"name": name, "user_id": user.user_id, "token": token})

route = ("/api/accounts/login", Login)
