from typing import TypedDict

from todo_app.utils import RequestHandler


class PostBody(TypedDict):
    username: str
    password: str

class Login(RequestHandler[None, PostBody, None, None, None]):
    async def post(self):
        name = self.post_body["username"]

        user = await self.db.query_single("""
            select User {
                name,
                id,
                password
            }
            filter .name = <str>$name
        """, name=name)

        if not user:
            return self.send_error(400, reason="incorrect login info")

        if not self.password_hasher.verify(self.post_body["password"], user.password):
            return self.send_error(400, reason="incorrect login info")

        token = self.tokens.create_token(str(user.id))

        self.finish({"name": name, "user_id": str(user.id), "token": token})

route = ("/api/accounts/login", Login)
