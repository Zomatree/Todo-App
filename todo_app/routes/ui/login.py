from todo_app.utils import RequestHandler

class Login(RequestHandler[None, None, None, None, None]):
    def get(self):
        self.render("login.html")

route = ("/login", Login, "login")
