from typing import TypedDict

from todo_app.utils import RequestHandler, create_id


class PostBody(TypedDict):
    username: str
    password: str

class CreateAccount(RequestHandler[None, PostBody, None, None, None]):
    async def post(self):
        if 2 > len(self.post_body["username"]) > 15:
            return self.send_error(400, reason="name must be between 2 and 15 characters")

        name = self.post_body["username"]
        id = create_id()
        hashed_password = self.password_hasher.hash(self.post_body["password"])

        await self.db.query("""
            insert User {
                name := <str>$name,
                user_id := <int64>$id,
                password := <str>$password
            }
        """, name=name, id=id, password=hashed_password)

        token = self.tokens.create_token(id)

        self.finish({"name": name, "user_id": id, "token": token})

route = ("/api/accounts/create", CreateAccount)
