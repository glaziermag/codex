"""Stub definitions for generated protobuf classes.

These are minimal placeholders so the package can be imported without
running `protoc`. For real usage generate proper classes as documented
in the README.
"""


class Task:
    def __init__(self, id: int = 0, name: str = ""):
        self.id = id
        self.name = name


class CreateTaskRequest:
    def __init__(self, name: str = ""):
        self.name = name


class GetTaskRequest:
    def __init__(self, id: int = 0):
        self.id = id


class StreamTasksRequest:
    def __init__(self, limit: int = 0):
        self.limit = limit
