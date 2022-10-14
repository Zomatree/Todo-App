import json
from types import NoneType
from typing import Any, Generic, Optional, TypedDict, TypeVar

from edgedb.asyncio_client import AsyncIOClient
import itsdangerous
from tornado.web import RequestHandler as _RequestHandler
from typing_extensions import NotRequired

from todo_app.utils.config import Config
from todo_app.utils.tokens import Tokens

from .passwords import PasswordHasher

T_GET = TypeVar("T_GET", bound=Optional[TypedDict])
T_POST = TypeVar("T_POST", bound=Optional[TypedDict])
T_PUT = TypeVar("T_PUT", bound=Optional[TypedDict])
T_PATCH = TypeVar("T_PATCH", bound=Optional[TypedDict])
T_DELETE = TypeVar("T_DELETE", bound=Optional[TypedDict])

class RequestHandler(_RequestHandler, Generic[T_GET, T_POST, T_PUT, T_PATCH, T_DELETE]):
    """Generic class for handling requests

    Generic Arguments:
        `[T_GET, T_POST, T_PUT, T_PATCH, T_DELETE]`
    """

    require_auth: bool

    get_parameters: T_GET
    post_body: T_POST
    put_body: T_PUT
    patch_body: T_PATCH
    delete_body: T_DELETE

    get_parameters_td: type | type[NoneType]
    post_body_td: type | type[NoneType]
    put_body_td: type | type[NoneType]
    patch_body_td: type | type[NoneType]
    delete_body_td: type | type[NoneType]

    def __init_subclass__(cls, *, require_auth: bool = False) -> None:
        cls.require_auth = require_auth

        args = cls.__orig_bases__[0].__args__  # type: ignore

        cls.get_parameters_td = args[0]
        cls.post_body_td = args[1]
        cls.put_body_td = args[2]
        cls.patch_body_td = args[3]
        cls.delete_body_td = args[4]

    @staticmethod
    def verify_body(body: dict[str, Any], td: type) -> bool:
        td_annotations = td.__annotations__
        print(body, td)

        for key, value in body.items():
            if key not in td_annotations:
                return False

            td_type = td_annotations[key]
            if origin := getattr(td_type, "__origin__", None):
                if origin is NotRequired:
                    td_type = td_type.__args__[0]

            print(key, value, td_type)

            if issubclass(td_type, list):
                if not isinstance(value, list):
                    return False

                generic = td_type.__args__[0]  # type: ignore

                if not all(isinstance(item, generic) for item in value):
                    return False

            # this is actually typeddict not dict, but at runtime typeddict is just a dict - i wont ever actually use bare `dict` in this

            if issubclass(td_type, dict):
                if not isinstance(value, dict):
                    return False

                if not RequestHandler.verify_body(value, td_type):
                    return False

            else:
                if not isinstance(value, td_type):
                    return False

        return True

    def initialize(self, *, db: AsyncIOClient, password_hasher: PasswordHasher, tokens: Tokens, config: Config):
        self.db = db
        self.password_hasher = password_hasher
        self.tokens = tokens
        self.config = config

    async def prepare(self):
        if self.request.method == "OPTIONS":
            return

        if self.require_auth:
            auth_header: str | None = self.request.headers.get("Authorization", None)

            if not auth_header:
                return self.send_error(401, reason="Missing required Authorization header")

            try:
                self.user_id = self.tokens.validate_token(auth_header)
            except itsdangerous.BadSignature:
                return self.send_error(401, reason="Invalid token")

        method = self.request.method

        if not method:
            return

        if method == "GET" and self.get_parameters_td is not NoneType:
            parameters = {k: v[0] for k, v in self.request.query_arguments.items()}

            if not self.verify_body(parameters, self.get_parameters_td):
                self.send_error(400, reason="Invalid parameters")

            return

        if self.request.method == "POST" and self.post_body_td is not NoneType:
            td = self.post_body_td
        elif self.request.method == "PUT" and self.put_body_td is not NoneType:
            td = self.put_body_td
        elif self.request.method == "PATCH" and self.patch_body_td is not NoneType:
            td = self.patch_body_td
        elif self.request.method == "DELETE" and self.delete_body_td is not NoneType:
            td = self.delete_body_td
        else:
            return

        try:
            body = json.loads(self.request.body)
        except json.JSONDecodeError:
            return self.send_error(400, reason="Invalid JSON")

        if not self.verify_body(body, td):
            return self.send_error(400, reason="Invalid body")

        setattr(self, f"{method.lower()}_body", body)

    def set_default_headers(self):
        self.set_header("access-control-allow-origin", "*")
        self.set_header("access-control-allow-methods", "GET, POST, OPTIONS, PUT, DELETE, PATCH")
        self.set_header("access-control-allow-headers", "content-type, authorization")

    async def options(self, *_):
        self.finish()
