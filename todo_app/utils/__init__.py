from typing import Any

from .passwords import PasswordHasher as PasswordHasher
from .route import RequestHandler as RequestHandler
from .tokens import Tokens as Tokens


def to_dict(obj: Any) -> dict[str, Any]:
    return {k: getattr(obj, k) for k in dir(obj)}
