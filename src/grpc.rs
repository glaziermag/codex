use tonic::{Request, Response, Status};

use crate::task_manager::{TaskManager, Task};
use crate::task::{task_service_server::{TaskService, TaskServiceServer}, *};

pub fn service(manager: TaskManager) -> TaskServiceServer<MyTaskService> {
    TaskServiceServer::new(MyTaskService { manager })
}

pub struct MyTaskService {
    manager: TaskManager,
}

#[tonic::async_trait]
impl TaskService for MyTaskService {
    async fn create_task(&self, request: Request<CreateTaskRequest>) -> Result<Response<TaskProto>, Status> {
        let title = request.into_inner().title;
        let task = self.manager.create_task(title);
        let reply = TaskProto { id: task.id, title: task.title };
        Ok(Response::new(reply))
    }

    async fn list_tasks(&self, _request: Request<ListTasksRequest>) -> Result<Response<TaskList>, Status> {
        let tasks = self.manager.list_tasks();
        let tasks_proto = tasks.into_iter().map(|t| TaskProto { id: t.id, title: t.title }).collect();
        let reply = TaskList { tasks: tasks_proto };
        Ok(Response::new(reply))
    }
}
