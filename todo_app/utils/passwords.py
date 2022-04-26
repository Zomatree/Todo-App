import argon2


class PasswordHasher:
    def __init__(self):
        self.hasher = argon2.PasswordHasher()

    def hash(self, password: str):
        return self.hasher.hash(password)

    def verify(self, password: str, hash: str) -> bool:
        try:
            return self.hasher.verify(hash, password)
        except:
            return False
