from __future__ import annotations

from typing import TypedDict

from typing_extensions import NotRequired


class Config(TypedDict):
    server: ServerConfig
    edgedb: EdgeDBConfig
    tokens: TokensConfig

class ServerConfig(TypedDict):
    host: str
    port: int

class EdgeDBConfig(TypedDict):
    host: NotRequired[str]
    port: NotRequired[int]
    user: NotRequired[str]
    password: NotRequired[str]
    database: NotRequired[str]

class TokensConfig(TypedDict):
    secret: str
