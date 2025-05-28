from concurrent import futures
from typing import Iterator

import grpc

from .store import PostgresStore
from .protos import task_pb2, task_pb2_grpc


class TaskService(task_pb2_grpc.TaskServiceServicer):
    """gRPC service for task operations."""

    def __init__(self, store: PostgresStore):
        self.store = store

    def CreateTask(self, request: task_pb2.CreateTaskRequest, context: grpc.ServicerContext) -> task_pb2.Task:
        task = self.store.add_task(request.name)
        return task_pb2.Task(id=task["id"], name=task["name"])

    def GetTask(self, request: task_pb2.GetTaskRequest, context: grpc.ServicerContext) -> task_pb2.Task:
        task = self.store.get_task(request.id)
        if not task:
            context.abort(grpc.StatusCode.NOT_FOUND, "Task not found")
        return task_pb2.Task(id=task["id"], name=task["name"])

    def StreamTasks(self, request: task_pb2.StreamTasksRequest, context: grpc.ServicerContext) -> Iterator[task_pb2.Task]:
        for t in self.store.list_tasks(limit=request.limit or None):
            yield task_pb2.Task(id=t["id"], name=t["name"])


def serve(address: str = "[::]:50051") -> grpc.Server:
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    store = PostgresStore()
    task_pb2_grpc.add_TaskServiceServicer_to_server(TaskService(store), server)
    server.add_insecure_port(address)
    server.start()
    return server


if __name__ == "__main__":  # pragma: no cover - manual invocation
    srv = serve()
    try:
        srv.wait_for_termination()
    except KeyboardInterrupt:
        srv.stop(0)
