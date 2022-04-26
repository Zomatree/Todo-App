from todo_app.utils import RequestHandler
from tornado.web import authenticated

class Login(RequestHandler[None, None, None, None, None]):
    @authenticated
    def get(self):
        self.render("todos.html")

route = ("/todos", Login)
