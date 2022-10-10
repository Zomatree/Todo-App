from typing import TypedDict

from todo_app.utils import RequestHandler


class PostBody(TypedDict):
    username: str
    password: str

class RegisterAccount(RequestHandler[None, PostBody, None, None, None]):
    async def post(self):
        if 2 > len(self.post_body["username"]) > 15:
            return self.send_error(400, reason="name must be between 2 and 15 characters")

        name = self.post_body["username"]
        hashed_password = self.password_hasher.hash(self.post_body["password"])

        user = await self.db.query_single("""
            insert User {
                name := <str>$name,
                password := <str>$password
            }
        """, name=name, password=hashed_password)

        token = self.tokens.create_token(str(user.id))

        self.finish({"name": name, "user_id": str(user.id), "token": token})

route = ("/api/accounts/register", RegisterAccount)
