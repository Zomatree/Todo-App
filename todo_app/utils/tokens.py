import base64

import itsdangerous


class Tokens:
    def __init__(self, secret: str):
        self.secret = secret
        self.signer = itsdangerous.Signer(secret)

    def create_token(self, id: int) -> str:
        encoded_id = base64.b64encode(str(id).encode())
        return self.signer.sign(encoded_id).decode()

    def validate_token(self, token: str) -> int:
        encoded_token = token.encode()
        data = self.signer.unsign(encoded_token)

        encoded_id = data.decode()
        return int(base64.b64decode(encoded_id).decode())
