use tonic::{Request, Response, Status};

use crate::task::v1::{
    task_service_server::{TaskService, TaskServiceServer},
    CreateTaskRequest, CreateTaskResponse, ListTasksRequest, ListTasksResponse,
    Task as TaskProto,
};
use crate::task_manager::TaskManager;
use tracing::instrument;

pub fn service(manager: TaskManager) -> TaskServiceServer<MyTaskService> {
    TaskServiceServer::new(MyTaskService { manager })
}

pub struct MyTaskService {
    manager: TaskManager,
}

#[tonic::async_trait]
impl TaskService for MyTaskService {
    #[instrument]
    async fn create_task(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<CreateTaskResponse>, Status> {
        let title = request.into_inner().title;
        let task = self.manager.create_task(title);
        let reply = CreateTaskResponse {
            task: Some(TaskProto {
                id: task.id,
                title: task.title,
            }),
        };
        Ok(Response::new(reply))
    }

    #[instrument]
    async fn list_tasks(
        &self,
        _request: Request<ListTasksRequest>,
    ) -> Result<Response<ListTasksResponse>, Status> {
        let tasks = self.manager.list_tasks();
        let tasks_proto = tasks
            .into_iter()
            .map(|t| TaskProto {
                id: t.id,
                title: t.title,
            })
            .collect();
        let reply = ListTasksResponse { tasks: tasks_proto };
        Ok(Response::new(reply))
    }
}
