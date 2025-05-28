"""Stub gRPC helpers for TaskService.

Real implementations should be generated with ``protoc`` and the
``grpc_python_plugin``.
"""

import grpc

from . import task_pb2


class TaskServiceServicer:
    """Abstract base class for TaskService."""

    def CreateTask(self, request, context):
        raise NotImplementedError()

    def GetTask(self, request, context):
        raise NotImplementedError()

    def StreamTasks(self, request, context):
        raise NotImplementedError()


def add_TaskServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {
        'CreateTask': grpc.unary_unary_rpc_method_handler(
            servicer.CreateTask,
        ),
        'GetTask': grpc.unary_unary_rpc_method_handler(
            servicer.GetTask,
        ),
        'StreamTasks': grpc.unary_stream_rpc_method_handler(
            servicer.StreamTasks,
        ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
        'taskd.TaskService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))
