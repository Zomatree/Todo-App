import importlib
import logging
import pathlib
from typing import Any, cast
import jinja2

import edgedb
import toml
from rich.logging import RichHandler
from tornado.ioloop import IOLoop
from tornado.web import Application, StaticFileHandler
from tornado.routing import URLSpec

from todo_app.utils.config import Config
from todo_app.utils.tokens import Tokens

from .utils import PasswordHasher

logging.basicConfig(format='%(message)s', level=logging.INFO, datefmt="[%X]", handlers=[RichHandler()])
log = logging.getLogger("todo_app")

cwd = pathlib.Path.cwd()
routes_path = cwd / 'todo_app' / 'routes'

class App(Application):
    def __init__(self, config: Config, db_client: edgedb.asyncio_client.AsyncIOClient):
        self.options = {
            "db": db_client,
            "password_hasher": PasswordHasher(),
            "tokens": Tokens(config["tokens"]["secret"]),
            "config": config,
            "environment": jinja2.Environment(loader=jinja2.FileSystemLoader(cwd / "templates")),
        }

        routes: list[Any] = []

        for path in routes_path.glob("**/*.py"):
            *parts, _ = path.relative_to(cwd).parts
            parts.append(path.stem)

            module = importlib.import_module(".".join(parts), package="todo_app")

            route: tuple = module.route
            name, handler, *extra = route

            routes.append(URLSpec(name, handler, self.options, *extra))
            log.info("Adding route: %s", route[0])

        routes.append(("/static/(.+)", StaticFileHandler, {"path": cwd / "static"}))

        super().__init__(routes)

    @classmethod
    def run(cls):
        with open("config.toml") as f:
            config = cast(Config, toml.load(f))

        db = edgedb.asyncio_client.create_async_client(**config["edgedb"])

        app = cls(config, db)

        port = config["server"]["port"]
        host = config["server"]["host"]

        log.info("Starting server on http://%s:%d", host, port)
        app.listen(port=port, address=host)

        IOLoop.current().start()
